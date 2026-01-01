/* FP:utils.rs-0001 */ use std::collections::HashMap;
/* FP:utils.rs-0002 */ use std::ffi::OsStr;
/* FP:utils.rs-0003 */ use std::fmt::Debug;
/* FP:utils.rs-0004 */ use std::fs;
/* FP:utils.rs-0005 */ #[cfg(unix)]
/* FP:utils.rs-0006 */ use std::os::unix::process::ExitStatusExt;
/* FP:utils.rs-0007 */ use std::path::{Path, PathBuf};
/* FP:utils.rs-0008 */ use std::process::{Command, ExitStatus, Output};
/* FP:utils.rs-0009 */ 
/* FP:utils.rs-0010 */ fn exec_command(
/* FP:utils.rs-0011 */     input: &[&dyn AsRef<OsStr>],
/* FP:utils.rs-0012 */     cwd: Option<&Path>,
/* FP:utils.rs-0013 */     env: Option<&HashMap<String, String>>,
/* FP:utils.rs-0014 */ ) -> Result<ExitStatus, String> {
/* FP:utils.rs-0015 */     let status = get_command_inner(input, cwd, env)
/* FP:utils.rs-0016 */         .spawn()
/* FP:utils.rs-0017 */         .map_err(|e| command_error(input, &cwd, e))?
/* FP:utils.rs-0018 */         .wait()
/* FP:utils.rs-0019 */         .map_err(|e| command_error(input, &cwd, e))?;
/* FP:utils.rs-0020 */     #[cfg(unix)]
/* FP:utils.rs-0021 */     {
/* FP:utils.rs-0022 */         if let Some(signal) = status.signal() {
/* FP:utils.rs-0023 */             // In case the signal didn't kill the current process.
/* FP:utils.rs-0024 */             return Err(command_error(input, &cwd, format!("Process received signal {signal}")));
/* FP:utils.rs-0025 */         }
/* FP:utils.rs-0026 */     }
/* FP:utils.rs-0027 */     Ok(status)
/* FP:utils.rs-0028 */ }
/* FP:utils.rs-0029 */ 
/* FP:utils.rs-0030 */ pub(crate) fn get_command_inner(
/* FP:utils.rs-0031 */     input: &[&dyn AsRef<OsStr>],
/* FP:utils.rs-0032 */     cwd: Option<&Path>,
/* FP:utils.rs-0033 */     env: Option<&HashMap<String, String>>,
/* FP:utils.rs-0034 */ ) -> Command {
/* FP:utils.rs-0035 */     let (cmd, args) = match input {
/* FP:utils.rs-0036 */         [] => panic!("empty command"),
/* FP:utils.rs-0037 */         [cmd, args @ ..] => (cmd, args),
/* FP:utils.rs-0038 */     };
/* FP:utils.rs-0039 */     let mut command = Command::new(cmd);
/* FP:utils.rs-0040 */     command.args(args);
/* FP:utils.rs-0041 */     if let Some(cwd) = cwd {
/* FP:utils.rs-0042 */         command.current_dir(cwd);
/* FP:utils.rs-0043 */     }
/* FP:utils.rs-0044 */     if let Some(env) = env {
/* FP:utils.rs-0045 */         command.envs(env.iter().map(|(k, v)| (k.as_str(), v.as_str())));
/* FP:utils.rs-0046 */     }
/* FP:utils.rs-0047 */     command
/* FP:utils.rs-0048 */ }
/* FP:utils.rs-0049 */ 
/* FP:utils.rs-0050 */ fn check_exit_status(
/* FP:utils.rs-0051 */     input: &[&dyn AsRef<OsStr>],
/* FP:utils.rs-0052 */     cwd: Option<&Path>,
/* FP:utils.rs-0053 */     exit_status: ExitStatus,
/* FP:utils.rs-0054 */     output: Option<&Output>,
/* FP:utils.rs-0055 */     show_err: bool,
/* FP:utils.rs-0056 */ ) -> Result<(), String> {
/* FP:utils.rs-0057 */     if exit_status.success() {
/* FP:utils.rs-0058 */         return Ok(());
/* FP:utils.rs-0059 */     }
/* FP:utils.rs-0060 */     let mut error = format!(
/* FP:utils.rs-0061 */         "Command `{}`{} exited with status {:?}",
/* FP:utils.rs-0062 */         input.iter().map(|s| s.as_ref().to_str().unwrap()).collect::<Vec<_>>().join(" "),
/* FP:utils.rs-0063 */         cwd.map(|cwd| format!(" (running in folder `{}`)", cwd.display())).unwrap_or_default(),
/* FP:utils.rs-0064 */         exit_status.code()
/* FP:utils.rs-0065 */     );
/* FP:utils.rs-0066 */     let input = input.iter().map(|i| i.as_ref()).collect::<Vec<&OsStr>>();
/* FP:utils.rs-0067 */     if show_err {
/* FP:utils.rs-0068 */         eprintln!("Command `{input:?}` failed");
/* FP:utils.rs-0069 */     }
/* FP:utils.rs-0070 */     if let Some(output) = output {
/* FP:utils.rs-0071 */         let stdout = String::from_utf8_lossy(&output.stdout);
/* FP:utils.rs-0072 */         if !stdout.is_empty() {
/* FP:utils.rs-0073 */             error.push_str("\n==== STDOUT ====\n");
/* FP:utils.rs-0074 */             error.push_str(&stdout);
/* FP:utils.rs-0075 */         }
/* FP:utils.rs-0076 */         let stderr = String::from_utf8_lossy(&output.stderr);
/* FP:utils.rs-0077 */         if !stderr.is_empty() {
/* FP:utils.rs-0078 */             error.push_str("\n==== STDERR ====\n");
/* FP:utils.rs-0079 */             error.push_str(&stderr);
/* FP:utils.rs-0080 */         }
/* FP:utils.rs-0081 */     }
/* FP:utils.rs-0082 */     Err(error)
/* FP:utils.rs-0083 */ }
/* FP:utils.rs-0084 */ 
/* FP:utils.rs-0085 */ fn command_error<D: Debug>(input: &[&dyn AsRef<OsStr>], cwd: &Option<&Path>, error: D) -> String {
/* FP:utils.rs-0086 */     format!(
/* FP:utils.rs-0087 */         "Command `{}`{} failed to run: {error:?}",
/* FP:utils.rs-0088 */         input.iter().map(|s| s.as_ref().to_str().unwrap()).collect::<Vec<_>>().join(" "),
/* FP:utils.rs-0089 */         cwd.as_ref()
/* FP:utils.rs-0090 */             .map(|cwd| format!(" (running in folder `{}`)", cwd.display(),))
/* FP:utils.rs-0091 */             .unwrap_or_default(),
/* FP:utils.rs-0092 */     )
/* FP:utils.rs-0093 */ }
/* FP:utils.rs-0094 */ 
/* FP:utils.rs-0095 */ pub fn run_command(input: &[&dyn AsRef<OsStr>], cwd: Option<&Path>) -> Result<Output, String> {
/* FP:utils.rs-0096 */     run_command_with_env(input, cwd, None)
/* FP:utils.rs-0097 */ }
/* FP:utils.rs-0098 */ 
/* FP:utils.rs-0099 */ pub fn run_command_with_env(
/* FP:utils.rs-0100 */     input: &[&dyn AsRef<OsStr>],
/* FP:utils.rs-0101 */     cwd: Option<&Path>,
/* FP:utils.rs-0102 */     env: Option<&HashMap<String, String>>,
/* FP:utils.rs-0103 */ ) -> Result<Output, String> {
/* FP:utils.rs-0104 */     let output =
/* FP:utils.rs-0105 */         get_command_inner(input, cwd, env).output().map_err(|e| command_error(input, &cwd, e))?;
/* FP:utils.rs-0106 */     check_exit_status(input, cwd, output.status, Some(&output), true)?;
/* FP:utils.rs-0107 */     Ok(output)
/* FP:utils.rs-0108 */ }
/* FP:utils.rs-0109 */ 
/* FP:utils.rs-0110 */ pub fn run_command_with_output(
/* FP:utils.rs-0111 */     input: &[&dyn AsRef<OsStr>],
/* FP:utils.rs-0112 */     cwd: Option<&Path>,
/* FP:utils.rs-0113 */ ) -> Result<(), String> {
/* FP:utils.rs-0114 */     let exit_status = exec_command(input, cwd, None)?;
/* FP:utils.rs-0115 */     check_exit_status(input, cwd, exit_status, None, true)?;
/* FP:utils.rs-0116 */     Ok(())
/* FP:utils.rs-0117 */ }
/* FP:utils.rs-0118 */ 
/* FP:utils.rs-0119 */ pub fn run_command_with_output_and_env(
/* FP:utils.rs-0120 */     input: &[&dyn AsRef<OsStr>],
/* FP:utils.rs-0121 */     cwd: Option<&Path>,
/* FP:utils.rs-0122 */     env: Option<&HashMap<String, String>>,
/* FP:utils.rs-0123 */ ) -> Result<(), String> {
/* FP:utils.rs-0124 */     let exit_status = exec_command(input, cwd, env)?;
/* FP:utils.rs-0125 */     check_exit_status(input, cwd, exit_status, None, true)?;
/* FP:utils.rs-0126 */     Ok(())
/* FP:utils.rs-0127 */ }
/* FP:utils.rs-0128 */ 
/* FP:utils.rs-0129 */ #[cfg(not(unix))]
/* FP:utils.rs-0130 */ pub fn run_command_with_output_and_env_no_err(
/* FP:utils.rs-0131 */     input: &[&dyn AsRef<OsStr>],
/* FP:utils.rs-0132 */     cwd: Option<&Path>,
/* FP:utils.rs-0133 */     env: Option<&HashMap<String, String>>,
/* FP:utils.rs-0134 */ ) -> Result<(), String> {
/* FP:utils.rs-0135 */     let exit_status = exec_command(input, cwd, env)?;
/* FP:utils.rs-0136 */     check_exit_status(input, cwd, exit_status, None, false)?;
/* FP:utils.rs-0137 */     Ok(())
/* FP:utils.rs-0138 */ }
/* FP:utils.rs-0139 */ 
/* FP:utils.rs-0140 */ pub fn cargo_install(to_install: &str) -> Result<(), String> {
/* FP:utils.rs-0141 */     let output = run_command(&[&"cargo", &"install", &"--list"], None)?;
/* FP:utils.rs-0142 */ 
/* FP:utils.rs-0143 */     let to_install_needle = format!("{to_install} ");
/* FP:utils.rs-0144 */     // cargo install --list returns something like this:
/* FP:utils.rs-0145 */     //
/* FP:utils.rs-0146 */     // mdbook-toc v0.8.0:
/* FP:utils.rs-0147 */     //     mdbook-toc
/* FP:utils.rs-0148 */     // rust-reduce v0.1.0:
/* FP:utils.rs-0149 */     //     rust-reduce
/* FP:utils.rs-0150 */     //
/* FP:utils.rs-0151 */     // We are only interested into the command name so we only look for lines ending with `:`.
/* FP:utils.rs-0152 */     if String::from_utf8(output.stdout)
/* FP:utils.rs-0153 */         .unwrap()
/* FP:utils.rs-0154 */         .lines()
/* FP:utils.rs-0155 */         .any(|line| line.ends_with(':') && line.starts_with(&to_install_needle))
/* FP:utils.rs-0156 */     {
/* FP:utils.rs-0157 */         return Ok(());
/* FP:utils.rs-0158 */     }
/* FP:utils.rs-0159 */     // We voluntarily ignore this error.
/* FP:utils.rs-0160 */     if run_command_with_output(&[&"cargo", &"install", &to_install], None).is_err() {
/* FP:utils.rs-0161 */         println!("Skipping installation of `{to_install}`");
/* FP:utils.rs-0162 */     }
/* FP:utils.rs-0163 */     Ok(())
/* FP:utils.rs-0164 */ }
/* FP:utils.rs-0165 */ 
/* FP:utils.rs-0166 */ pub fn get_os_name() -> Result<String, String> {
/* FP:utils.rs-0167 */     let output = run_command(&[&"uname"], None)?;
/* FP:utils.rs-0168 */     let name = std::str::from_utf8(&output.stdout).unwrap_or("").trim().to_string();
/* FP:utils.rs-0169 */     if !name.is_empty() { Ok(name) } else { Err("Failed to retrieve the OS name".to_string()) }
/* FP:utils.rs-0170 */ }
/* FP:utils.rs-0171 */ 
/* FP:utils.rs-0172 */ #[derive(Default, PartialEq)]
/* FP:utils.rs-0173 */ pub struct RustcVersionInfo {
/* FP:utils.rs-0174 */     pub short: String,
/* FP:utils.rs-0175 */     pub version: String,
/* FP:utils.rs-0176 */     pub host: Option<String>,
/* FP:utils.rs-0177 */     pub commit_hash: Option<String>,
/* FP:utils.rs-0178 */     pub commit_date: Option<String>,
/* FP:utils.rs-0179 */ }
/* FP:utils.rs-0180 */ 
/* FP:utils.rs-0181 */ pub fn rustc_toolchain_version_info(toolchain: &str) -> Result<RustcVersionInfo, String> {
/* FP:utils.rs-0182 */     rustc_version_info_inner(None, Some(toolchain))
/* FP:utils.rs-0183 */ }
/* FP:utils.rs-0184 */ 
/* FP:utils.rs-0185 */ pub fn rustc_version_info(rustc: Option<&str>) -> Result<RustcVersionInfo, String> {
/* FP:utils.rs-0186 */     rustc_version_info_inner(rustc, None)
/* FP:utils.rs-0187 */ }
/* FP:utils.rs-0188 */ 
/* FP:utils.rs-0189 */ fn rustc_version_info_inner(
/* FP:utils.rs-0190 */     rustc: Option<&str>,
/* FP:utils.rs-0191 */     toolchain: Option<&str>,
/* FP:utils.rs-0192 */ ) -> Result<RustcVersionInfo, String> {
/* FP:utils.rs-0193 */     let output = if let Some(toolchain) = toolchain {
/* FP:utils.rs-0194 */         run_command(&[&rustc.unwrap_or("rustc"), &toolchain, &"-vV"], None)
/* FP:utils.rs-0195 */     } else {
/* FP:utils.rs-0196 */         run_command(&[&rustc.unwrap_or("rustc"), &"-vV"], None)
/* FP:utils.rs-0197 */     }?;
/* FP:utils.rs-0198 */     let content = std::str::from_utf8(&output.stdout).unwrap_or("");
/* FP:utils.rs-0199 */ 
/* FP:utils.rs-0200 */     let mut info = RustcVersionInfo::default();
/* FP:utils.rs-0201 */     let mut lines = content.split('\n');
/* FP:utils.rs-0202 */     info.short = match lines.next() {
/* FP:utils.rs-0203 */         Some(s) => s.to_string(),
/* FP:utils.rs-0204 */         None => return Err("failed to retrieve rustc version".to_string()),
/* FP:utils.rs-0205 */     };
/* FP:utils.rs-0206 */ 
/* FP:utils.rs-0207 */     for line in lines.map(|line| line.trim()) {
/* FP:utils.rs-0208 */         match line.split_once(':') {
/* FP:utils.rs-0209 */             Some(("host", data)) => info.host = Some(data.trim().to_string()),
/* FP:utils.rs-0210 */             Some(("release", data)) => info.version = data.trim().to_string(),
/* FP:utils.rs-0211 */             Some(("commit-hash", data)) => info.commit_hash = Some(data.trim().to_string()),
/* FP:utils.rs-0212 */             Some(("commit-date", data)) => info.commit_date = Some(data.trim().to_string()),
/* FP:utils.rs-0213 */             _ => {}
/* FP:utils.rs-0214 */         }
/* FP:utils.rs-0215 */     }
/* FP:utils.rs-0216 */     if info.version.is_empty() {
/* FP:utils.rs-0217 */         Err("failed to retrieve rustc version".to_string())
/* FP:utils.rs-0218 */     } else {
/* FP:utils.rs-0219 */         Ok(info)
/* FP:utils.rs-0220 */     }
/* FP:utils.rs-0221 */ }
/* FP:utils.rs-0222 */ 
/* FP:utils.rs-0223 */ pub fn get_toolchain() -> Result<String, String> {
/* FP:utils.rs-0224 */     let content = match fs::read_to_string("rust-toolchain") {
/* FP:utils.rs-0225 */         Ok(content) => content,
/* FP:utils.rs-0226 */         Err(_) => return Err("No `rust-toolchain` file found".to_string()),
/* FP:utils.rs-0227 */     };
/* FP:utils.rs-0228 */     match content
/* FP:utils.rs-0229 */         .split('\n')
/* FP:utils.rs-0230 */         .map(|line| line.trim())
/* FP:utils.rs-0231 */         .filter(|line| !line.is_empty())
/* FP:utils.rs-0232 */         .filter_map(|line| {
/* FP:utils.rs-0233 */             if !line.starts_with("channel") {
/* FP:utils.rs-0234 */                 return None;
/* FP:utils.rs-0235 */             }
/* FP:utils.rs-0236 */             line.split('"').nth(1)
/* FP:utils.rs-0237 */         })
/* FP:utils.rs-0238 */         .next()
/* FP:utils.rs-0239 */     {
/* FP:utils.rs-0240 */         Some(toolchain) => Ok(toolchain.to_string()),
/* FP:utils.rs-0241 */         None => Err("Couldn't find `channel` in `rust-toolchain` file".to_string()),
/* FP:utils.rs-0242 */     }
/* FP:utils.rs-0243 */ }
/* FP:utils.rs-0244 */ 
/* FP:utils.rs-0245 */ pub struct CloneResult {
/* FP:utils.rs-0246 */     pub ran_clone: bool,
/* FP:utils.rs-0247 */     pub repo_name: String,
/* FP:utils.rs-0248 */     pub repo_dir: String,
/* FP:utils.rs-0249 */ }
/* FP:utils.rs-0250 */ 
/* FP:utils.rs-0251 */ fn git_clone_inner(
/* FP:utils.rs-0252 */     to_clone: &str,
/* FP:utils.rs-0253 */     dest: &Path,
/* FP:utils.rs-0254 */     shallow_clone: bool,
/* FP:utils.rs-0255 */     repo_name: String,
/* FP:utils.rs-0256 */ ) -> Result<CloneResult, String> {
/* FP:utils.rs-0257 */     if dest.is_dir() {
/* FP:utils.rs-0258 */         return Ok(CloneResult {
/* FP:utils.rs-0259 */             ran_clone: false,
/* FP:utils.rs-0260 */             repo_name,
/* FP:utils.rs-0261 */             repo_dir: dest.display().to_string(),
/* FP:utils.rs-0262 */         });
/* FP:utils.rs-0263 */     }
/* FP:utils.rs-0264 */ 
/* FP:utils.rs-0265 */     let mut command: Vec<&dyn AsRef<OsStr>> = vec![&"git", &"clone", &to_clone, &dest];
/* FP:utils.rs-0266 */     if shallow_clone {
/* FP:utils.rs-0267 */         command.push(&"--depth");
/* FP:utils.rs-0268 */         command.push(&"1");
/* FP:utils.rs-0269 */     }
/* FP:utils.rs-0270 */     run_command_with_output(&command, None)?;
/* FP:utils.rs-0271 */     Ok(CloneResult { ran_clone: true, repo_name, repo_dir: dest.display().to_string() })
/* FP:utils.rs-0272 */ }
/* FP:utils.rs-0273 */ 
/* FP:utils.rs-0274 */ fn get_repo_name(url: &str) -> String {
/* FP:utils.rs-0275 */     let repo_name = url.split('/').next_back().unwrap();
/* FP:utils.rs-0276 */     match repo_name.strip_suffix(".git") {
/* FP:utils.rs-0277 */         Some(n) => n.to_string(),
/* FP:utils.rs-0278 */         None => repo_name.to_string(),
/* FP:utils.rs-0279 */     }
/* FP:utils.rs-0280 */ }
/* FP:utils.rs-0281 */ 
/* FP:utils.rs-0282 */ pub fn git_clone(
/* FP:utils.rs-0283 */     to_clone: &str,
/* FP:utils.rs-0284 */     dest: Option<&Path>,
/* FP:utils.rs-0285 */     shallow_clone: bool,
/* FP:utils.rs-0286 */ ) -> Result<CloneResult, String> {
/* FP:utils.rs-0287 */     let repo_name = get_repo_name(to_clone);
/* FP:utils.rs-0288 */     let tmp: PathBuf;
/* FP:utils.rs-0289 */ 
/* FP:utils.rs-0290 */     let dest = match dest {
/* FP:utils.rs-0291 */         Some(dest) => dest,
/* FP:utils.rs-0292 */         None => {
/* FP:utils.rs-0293 */             tmp = repo_name.clone().into();
/* FP:utils.rs-0294 */             &tmp
/* FP:utils.rs-0295 */         }
/* FP:utils.rs-0296 */     };
/* FP:utils.rs-0297 */     git_clone_inner(to_clone, dest, shallow_clone, repo_name)
/* FP:utils.rs-0298 */ }
/* FP:utils.rs-0299 */ 
/* FP:utils.rs-0300 */ pub fn create_dir<P: AsRef<Path>>(path: P) -> Result<(), String> {
/* FP:utils.rs-0301 */     fs::create_dir_all(&path).map_err(|error| {
/* FP:utils.rs-0302 */         format!("Failed to create directory `{}`: {:?}", path.as_ref().display(), error)
/* FP:utils.rs-0303 */     })
/* FP:utils.rs-0304 */ }
/* FP:utils.rs-0305 */ 
/* FP:utils.rs-0306 */ /// This function differs from `git_clone` in how it handles *where* the repository will be cloned.
/* FP:utils.rs-0307 */ /// In `git_clone`, it is cloned in the provided path. In this function, the path you provide is
/* FP:utils.rs-0308 */ /// the parent folder. So if you pass "a" as folder and try to clone "b.git", it will be cloned into
/* FP:utils.rs-0309 */ /// `a/b`.
/* FP:utils.rs-0310 */ pub fn git_clone_root_dir(
/* FP:utils.rs-0311 */     to_clone: &str,
/* FP:utils.rs-0312 */     dest_parent_dir: &Path,
/* FP:utils.rs-0313 */     shallow_clone: bool,
/* FP:utils.rs-0314 */ ) -> Result<CloneResult, String> {
/* FP:utils.rs-0315 */     let repo_name = get_repo_name(to_clone);
/* FP:utils.rs-0316 */ 
/* FP:utils.rs-0317 */     git_clone_inner(to_clone, &dest_parent_dir.join(&repo_name), shallow_clone, repo_name)
/* FP:utils.rs-0318 */ }
/* FP:utils.rs-0319 */ 
/* FP:utils.rs-0320 */ pub fn walk_dir<P, D, F>(
/* FP:utils.rs-0321 */     dir: P,
/* FP:utils.rs-0322 */     dir_cb: &mut D,
/* FP:utils.rs-0323 */     file_cb: &mut F,
/* FP:utils.rs-0324 */     recursive: bool,
/* FP:utils.rs-0325 */ ) -> Result<(), String>
/* FP:utils.rs-0326 */ where
/* FP:utils.rs-0327 */     P: AsRef<Path>,
/* FP:utils.rs-0328 */     D: FnMut(&Path) -> Result<(), String>,
/* FP:utils.rs-0329 */     F: FnMut(&Path) -> Result<(), String>,
/* FP:utils.rs-0330 */ {
/* FP:utils.rs-0331 */     let dir = dir.as_ref();
/* FP:utils.rs-0332 */     for entry in fs::read_dir(dir)
/* FP:utils.rs-0333 */         .map_err(|error| format!("Failed to read dir `{}`: {:?}", dir.display(), error))?
/* FP:utils.rs-0334 */     {
/* FP:utils.rs-0335 */         let entry = entry
/* FP:utils.rs-0336 */             .map_err(|error| format!("Failed to read entry in `{}`: {:?}", dir.display(), error))?;
/* FP:utils.rs-0337 */         let entry_path = entry.path();
/* FP:utils.rs-0338 */         if entry_path.is_dir() {
/* FP:utils.rs-0339 */             dir_cb(&entry_path)?;
/* FP:utils.rs-0340 */             if recursive {
/* FP:utils.rs-0341 */                 walk_dir(entry_path, dir_cb, file_cb, recursive)?; // Recursive call
/* FP:utils.rs-0342 */             }
/* FP:utils.rs-0343 */         } else {
/* FP:utils.rs-0344 */             file_cb(&entry_path)?;
/* FP:utils.rs-0345 */         }
/* FP:utils.rs-0346 */     }
/* FP:utils.rs-0347 */     Ok(())
/* FP:utils.rs-0348 */ }
/* FP:utils.rs-0349 */ 
/* FP:utils.rs-0350 */ pub fn split_args(args: &str) -> Result<Vec<String>, String> {
/* FP:utils.rs-0351 */     let mut out = Vec::new();
/* FP:utils.rs-0352 */     let mut start = 0;
/* FP:utils.rs-0353 */     let args = args.trim();
/* FP:utils.rs-0354 */     let mut iter = args.char_indices().peekable();
/* FP:utils.rs-0355 */ 
/* FP:utils.rs-0356 */     while let Some((pos, c)) = iter.next() {
/* FP:utils.rs-0357 */         if c == ' ' {
/* FP:utils.rs-0358 */             out.push(args[start..pos].to_string());
/* FP:utils.rs-0359 */             let mut found_start = false;
/* FP:utils.rs-0360 */             while let Some((pos, c)) = iter.peek() {
/* FP:utils.rs-0361 */                 if *c != ' ' {
/* FP:utils.rs-0362 */                     start = *pos;
/* FP:utils.rs-0363 */                     found_start = true;
/* FP:utils.rs-0364 */                     break;
/* FP:utils.rs-0365 */                 } else {
/* FP:utils.rs-0366 */                     iter.next();
/* FP:utils.rs-0367 */                 }
/* FP:utils.rs-0368 */             }
/* FP:utils.rs-0369 */             if !found_start {
/* FP:utils.rs-0370 */                 return Ok(out);
/* FP:utils.rs-0371 */             }
/* FP:utils.rs-0372 */         } else if c == '"' || c == '\'' {
/* FP:utils.rs-0373 */             let end = c;
/* FP:utils.rs-0374 */             let mut found_end = false;
/* FP:utils.rs-0375 */             while let Some((_, c)) = iter.next() {
/* FP:utils.rs-0376 */                 if c == end {
/* FP:utils.rs-0377 */                     found_end = true;
/* FP:utils.rs-0378 */                     break;
/* FP:utils.rs-0379 */                 } else if c == '\\' {
/* FP:utils.rs-0380 */                     // We skip the escaped character.
/* FP:utils.rs-0381 */                     iter.next();
/* FP:utils.rs-0382 */                 }
/* FP:utils.rs-0383 */             }
/* FP:utils.rs-0384 */             if !found_end {
/* FP:utils.rs-0385 */                 return Err(format!("Didn't find `{}` at the end of `{}`", end, &args[start..]));
/* FP:utils.rs-0386 */             }
/* FP:utils.rs-0387 */         } else if c == '\\' {
/* FP:utils.rs-0388 */             // We skip the escaped character.
/* FP:utils.rs-0389 */             iter.next();
/* FP:utils.rs-0390 */         }
/* FP:utils.rs-0391 */     }
/* FP:utils.rs-0392 */     let s = args[start..].trim();
/* FP:utils.rs-0393 */     if !s.is_empty() {
/* FP:utils.rs-0394 */         out.push(s.to_string());
/* FP:utils.rs-0395 */     }
/* FP:utils.rs-0396 */     Ok(out)
/* FP:utils.rs-0397 */ }
/* FP:utils.rs-0398 */ 
/* FP:utils.rs-0399 */ pub fn remove_file<P: AsRef<Path> + ?Sized>(file_path: &P) -> Result<(), String> {
/* FP:utils.rs-0400 */     std::fs::remove_file(file_path).map_err(|error| {
/* FP:utils.rs-0401 */         format!("Failed to remove `{}`: {:?}", file_path.as_ref().display(), error)
/* FP:utils.rs-0402 */     })
/* FP:utils.rs-0403 */ }
/* FP:utils.rs-0404 */ 
/* FP:utils.rs-0405 */ pub fn create_symlink<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> Result<(), String> {
/* FP:utils.rs-0406 */     #[cfg(windows)]
/* FP:utils.rs-0407 */     let symlink = std::os::windows::fs::symlink_file;
/* FP:utils.rs-0408 */     #[cfg(not(windows))]
/* FP:utils.rs-0409 */     let symlink = std::os::unix::fs::symlink;
/* FP:utils.rs-0410 */ 
/* FP:utils.rs-0411 */     symlink(&original, &link).map_err(|err| {
/* FP:utils.rs-0412 */         format!(
/* FP:utils.rs-0413 */             "failed to create a symlink `{}` to `{}`: {:?}",
/* FP:utils.rs-0414 */             original.as_ref().display(),
/* FP:utils.rs-0415 */             link.as_ref().display(),
/* FP:utils.rs-0416 */             err,
/* FP:utils.rs-0417 */         )
/* FP:utils.rs-0418 */     })
/* FP:utils.rs-0419 */ }
/* FP:utils.rs-0420 */ 
/* FP:utils.rs-0421 */ pub fn get_sysroot_dir() -> PathBuf {
/* FP:utils.rs-0422 */     Path::new(crate::BUILD_DIR).join("build_sysroot")
/* FP:utils.rs-0423 */ }
/* FP:utils.rs-0424 */ 
/* FP:utils.rs-0425 */ #[cfg(test)]
/* FP:utils.rs-0426 */ mod tests {
/* FP:utils.rs-0427 */     use super::*;
/* FP:utils.rs-0428 */ 
/* FP:utils.rs-0429 */     #[test]
/* FP:utils.rs-0430 */     fn test_split_args() {
/* FP:utils.rs-0431 */         // Missing `"` at the end.
/* FP:utils.rs-0432 */         assert!(split_args("\"tada").is_err());
/* FP:utils.rs-0433 */         // Missing `'` at the end.
/* FP:utils.rs-0434 */         assert!(split_args("\'tada").is_err());
/* FP:utils.rs-0435 */ 
/* FP:utils.rs-0436 */         assert_eq!(
/* FP:utils.rs-0437 */             split_args("a \"b\" c"),
/* FP:utils.rs-0438 */             Ok(vec!["a".to_string(), "\"b\"".to_string(), "c".to_string()])
/* FP:utils.rs-0439 */         );
/* FP:utils.rs-0440 */         // Trailing whitespace characters.
/* FP:utils.rs-0441 */         assert_eq!(
/* FP:utils.rs-0442 */             split_args("    a    \"b\" c    "),
/* FP:utils.rs-0443 */             Ok(vec!["a".to_string(), "\"b\"".to_string(), "c".to_string()])
/* FP:utils.rs-0444 */         );
/* FP:utils.rs-0445 */     }
/* FP:utils.rs-0446 */ }