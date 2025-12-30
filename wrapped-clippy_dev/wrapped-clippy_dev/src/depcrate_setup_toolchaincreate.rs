// Generated macro for create (function)
macro_rules! Depcrate_setup_toolchaincreate {
() => {
// Module: crate::setup::toolchain
// Provides: {"create"}
// Dependencies: {}
pub fn create (standalone : bool , force : bool , release : bool , name : & str) { let rustup_home = std :: env :: var ("RUSTUP_HOME") . unwrap () ; let toolchain = std :: env :: var ("RUSTUP_TOOLCHAIN") . unwrap () ; let src = PathBuf :: from_iter ([& rustup_home , "toolchains" , & toolchain]) ; let dest = PathBuf :: from_iter ([& rustup_home , "toolchains" , name]) ; if dest . exists () { if force { fs :: remove_dir_all (& dest) . unwrap () ; } else { println ! ("{} already exists, pass `--force` to override it" , dest . display ()) ; return ; } } for entry in WalkDir :: new (& src) { let entry = entry . unwrap () ; let relative = entry . path () . strip_prefix (& src) . unwrap () ; if relative . starts_with ("bin") && matches ! (relative . file_stem () . and_then (OsStr :: to_str) , Some ("cargo-clippy" | "clippy-driver")) { continue ; } let target = dest . join (relative) ; if entry . file_type () . is_dir () { fs :: create_dir (& target) . unwrap () ; } else { fs :: hard_link (entry . path () , target) . unwrap () ; } } run_exit_on_err ("cargo build" , cargo_cmd () . arg ("build") . args (release . then_some ("--release")) ,) ; install_bin ("cargo-clippy" , & dest , standalone , release) ; install_bin ("clippy-driver" , & dest , standalone , release) ; println ! ("Created toolchain {name}, use it in other projects with e.g. `cargo +{name} clippy`") ; if ! standalone { println ! ("Note: This will need to be re-run whenever the Clippy `rust-toolchain.toml` changes") ; } }
};
}
