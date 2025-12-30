// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl < 'a > SysrootBuilder < 'a > { # [doc = " Prepare to create a new sysroot in the given folder (that folder should later be passed to"] # [doc = " rustc via `--sysroot`), for the given target."] pub fn new (sysroot_dir : & Path , target : impl Into < OsString >) -> Self { let default_flags = & ["-Zforce-unstable-if-unmarked" , "--cap-lints=warn" , "-Aunexpected_cfgs" ,] ; SysrootBuilder { sysroot_dir : sysroot_dir . to_owned () , target : target . into () , config : SysrootConfig :: WithStd { std_features : vec ! [] , } , mode : BuildMode :: Build , rustflags : default_flags . iter () . map (Into :: into) . collect () , cargo : None , rustc_version : None , when_build_required : None , } } # [doc = " Sets the build mode (regular build vs check-only build)."] pub fn build_mode (mut self , build_mode : BuildMode) -> Self { self . mode = build_mode ; self } # [doc = " Sets the sysroot configuration (which parts of the sysroot to build and with which features)."] pub fn sysroot_config (mut self , sysroot_config : SysrootConfig) -> Self { self . config = sysroot_config ; self } # [doc = " Appends the given flag."] # [doc = ""] # [doc = " If no `--cap-lints` argument is configured, we will add `--cap-lints=warn`."] # [doc = " This emulates the usual behavior of Cargo: Lints are normally capped when building"] # [doc = " dependencies, except that they are not capped when building path dependencies, except that"] # [doc = " path dependencies are still capped if they are part of `-Zbuild-std`."] pub fn rustflag (mut self , rustflag : impl Into < OsString >) -> Self { self . rustflags . push (rustflag . into ()) ; self } # [doc = " Appends the given flags."] # [doc = ""] # [doc = " If no `--cap-lints` argument is configured, we will add `--cap-lints=warn`. See"] # [doc = " [`SysrootBuilder::rustflag`] for more explanation."] pub fn rustflags (mut self , rustflags : impl IntoIterator < Item = impl Into < OsString > >) -> Self { self . rustflags . extend (rustflags . into_iter () . map (Into :: into)) ; self } # [doc = " Sets the cargo command to call."] # [doc = ""] # [doc = " This will be invoked with `output()`, so if stdout/stderr should be inherited"] # [doc = " then that needs to be set explicitly."] pub fn cargo (mut self , cargo : Command) -> Self { self . cargo = Some (cargo) ; self } # [doc = " Sets the rustc version information (in case the user has that available)."] pub fn rustc_version (mut self , rustc_version : rustc_version :: VersionMeta) -> Self { self . rustc_version = Some (rustc_version) ; self } # [doc = " Sets the hook that will be called if we don't have a cached sysroot available and a new one"] # [doc = " will be compiled."] pub fn when_build_required (mut self , when_build_required : impl FnOnce () + 'a) -> Self { self . when_build_required = Some (Box :: new (when_build_required)) ; self } # [doc = " Our configured target can be either a built-in target name, or a path to a target file."] # [doc = " We use the same logic as rustc to tell which is which:"] # [doc = " https://github.com/rust-lang/rust/blob/8d39ec1825024f3014e1f847942ac5bbfcf055b0/compiler/rustc_session/src/config.rs#L2252-L2263"] fn target_name (& self) -> & OsStr { let path = Path :: new (& self . target) ; if path . extension () . and_then (OsStr :: to_str) == Some ("json") { path . file_stem () . unwrap () } else { & self . target } } fn sysroot_target_dir (& self) -> PathBuf { self . sysroot_dir . join ("lib") . join ("rustlib") . join (self . target_name ()) } # [doc = " Computes the hash for the sysroot, so that we know whether we have to rebuild."] fn sysroot_compute_hash (& self , src_dir : & Path , rustc_version : & rustc_version :: VersionMeta ,) -> Result < u64 > { let mut hasher = DefaultHasher :: new () ; src_dir . hash (& mut hasher) ; hash_recursive (src_dir , & mut hasher) ? ; self . config . hash (& mut hasher) ; self . mode . hash (& mut hasher) ; self . rustflags . hash (& mut hasher) ; rustc_version . hash (& mut hasher) ; Ok (hasher . finish ()) } fn sysroot_read_hash (& self) -> Option < u64 > { let hash_file = self . sysroot_target_dir () . join (HASH_FILE_NAME) ; let hash = fs :: read_to_string (& hash_file) . ok () ? ; hash . parse () . ok () } # [doc = " Generate the contents of the manifest file for the sysroot build."] fn gen_manifest (& self , src_dir : & Path) -> String { let have_sysroot_crate = src_dir . join ("sysroot") . exists () ; let crates = match & self . config { SysrootConfig :: NoStd => format ! (r#"
[dependencies.core]
path = {src_dir_core:?}
[dependencies.alloc]
path = {src_dir_alloc:?}
[dependencies.compiler_builtins]
features = ["rustc-dep-of-std", "mem"]
version = "*"
                "# , src_dir_core = src_dir . join ("core") , src_dir_alloc = src_dir . join ("alloc") ,) , SysrootConfig :: WithStd { std_features } if have_sysroot_crate => format ! (r#"
[dependencies.std]
features = {std_features:?}
path = {src_dir_std:?}
[dependencies.sysroot]
path = {src_dir_sysroot:?}
                "# , std_features = std_features , src_dir_std = src_dir . join ("std") , src_dir_sysroot = src_dir . join ("sysroot") ,) , SysrootConfig :: WithStd { std_features } => format ! (r#"
[dependencies.std]
features = {std_features:?}
path = {src_dir_std:?}
[dependencies.test]
path = {src_dir_test:?}
                "# , std_features = std_features , src_dir_std = src_dir . join ("std") , src_dir_test = src_dir . join ("test") ,) , } ; let patches = match & self . config { SysrootConfig :: NoStd => format ! (r#"
[patch.crates-io.rustc-std-workspace-core]
path = {src_dir_workspace_core:?}
                "# , src_dir_workspace_core = src_dir . join ("rustc-std-workspace-core") ,) , SysrootConfig :: WithStd { .. } => format ! (r#"
[patch.crates-io.rustc-std-workspace-core]
path = {src_dir_workspace_core:?}
[patch.crates-io.rustc-std-workspace-alloc]
path = {src_dir_workspace_alloc:?}
[patch.crates-io.rustc-std-workspace-std]
path = {src_dir_workspace_std:?}
                "# , src_dir_workspace_core = src_dir . join ("rustc-std-workspace-core") , src_dir_workspace_alloc = src_dir . join ("rustc-std-workspace-alloc") , src_dir_workspace_std = src_dir . join ("rustc-std-workspace-std") ,) , } ; format ! (r#"
[package]
authors = ["rustc-build-sysroot"]
name = "custom-local-sysroot"
version = "0.0.0"

[lib]
# empty dummy, just so that things are being built
path = "lib.rs"

[profile.{DEFAULT_SYSROOT_PROFILE}]
# We inherit from the local release profile, but then overwrite some
# settings to ensure we still get a working sysroot.
inherits = "release"
panic = 'unwind'

{crates}

{patches}

[patch.crates-io]
compiler_builtins = {{ git = "https://github.com/anza-xyz/compiler-builtins", tag = "solana-tools-v1.51" }}
            "#) } # [doc = " Build the `self` sysroot from the given sources."] # [doc = ""] # [doc = " `src_dir` must be the `library` source folder, i.e., the one that contains `std/Cargo.toml`."] pub fn build_from_source (mut self , src_dir : & Path) -> Result < SysrootStatus > { if ! src_dir . join ("std") . join ("Cargo.toml") . exists () { bail ! ("{:?} does not seem to be a rust library source folder: `src/Cargo.toml` not found" , src_dir) ; } let sysroot_target_dir = self . sysroot_target_dir () ; let target_name = self . target_name () . to_owned () ; let cargo = self . cargo . take () . unwrap_or_else (| | { Command :: new (env :: var_os ("CARGO") . unwrap_or_else (| | OsString :: from ("cargo"))) }) ; let rustc_version = match self . rustc_version . take () { Some (v) => v , None => rustc_version :: version_meta () ? , } ; let cur_hash = self . sysroot_compute_hash (src_dir , & rustc_version) ? ; if self . sysroot_read_hash () == Some (cur_hash) { return Ok (SysrootStatus :: AlreadyCached) ; } if let Some (when_build_required) = self . when_build_required . take () { when_build_required () ; } let build_dir = TempDir :: new () . context ("failed to create tempdir") ? ; let lock_file = build_dir . path () . join ("Cargo.lock") ; let lock_file_src = { let new_lock_file_name = src_dir . join ("Cargo.lock") ; if new_lock_file_name . exists () { new_lock_file_name } else { src_dir . parent () . expect ("src_dir must have a parent") . join ("Cargo.lock") } } ; fs :: copy (lock_file_src , & lock_file) . context ("failed to copy lockfile from sysroot source") ? ; make_writeable (& lock_file) . context ("failed to make lockfile writeable") ? ; let manifest_file = build_dir . path () . join ("Cargo.toml") ; let manifest = self . gen_manifest (src_dir) ; fs :: write (& manifest_file , manifest . as_bytes ()) . context ("failed to write manifest file") ? ; let lib_file = build_dir . path () . join ("lib.rs") ; let lib = match self . config { SysrootConfig :: NoStd => r#"#![no_std]"# , SysrootConfig :: WithStd { .. } => "" , } ; fs :: write (& lib_file , lib . as_bytes ()) . context ("failed to write lib file") ? ; let mut cmd = cargo ; cmd . arg (self . mode . as_str ()) ; cmd . arg ("--profile") ; cmd . arg (DEFAULT_SYSROOT_PROFILE) ; cmd . arg ("--manifest-path") ; cmd . arg (& manifest_file) ; cmd . arg ("--target") ; cmd . arg (& self . target) ; cmd . env ("CARGO_ENCODED_RUSTFLAGS" , encode_rustflags (& self . rustflags)) ; let build_target_dir = build_dir . path () . join ("target") ; cmd . env ("CARGO_TARGET_DIR" , & build_target_dir) ; cmd . env ("__CARGO_DEFAULT_LIB_METADATA" , "rustc-build-sysroot") ; let output = cmd . output () . context ("failed to execute cargo for sysroot build") ? ; if ! output . status . success () { let stderr = String :: from_utf8_lossy (& output . stderr) ; if stderr . is_empty () { bail ! ("sysroot build failed") ; } else { bail ! ("sysroot build failed; stderr:\n{}" , stderr) ; } } fs :: create_dir_all (& self . sysroot_dir) . context ("failed to create sysroot dir") ? ; let staging_dir = TempDir :: new_in (& self . sysroot_dir) . context ("failed to create staging dir") ? ; let staging_lib_dir = staging_dir . path () . join ("lib") ; fs :: create_dir (& staging_lib_dir) . context ("faiked to create staging/lib dir") ? ; let out_dir = build_target_dir . join (& target_name) . join (DEFAULT_SYSROOT_PROFILE) . join ("deps") ; for entry in fs :: read_dir (& out_dir) . context ("failed to read cargo out dir") ? { let entry = entry . context ("failed to read cargo out dir entry") ? ; assert ! (entry . file_type () . unwrap () . is_file () , "cargo out dir must not contain directories") ; let entry = entry . path () ; fs :: copy (& entry , staging_lib_dir . join (entry . file_name () . unwrap ())) . context ("failed to copy cargo out file") ? ; } fs :: write (staging_dir . path () . join (HASH_FILE_NAME) , cur_hash . to_string () . as_bytes () ,) . context ("failed to write hash file") ? ; if sysroot_target_dir . exists () { fs :: remove_dir_all (& sysroot_target_dir) . context ("failed to clean sysroot target dir") ? ; } fs :: create_dir_all (& sysroot_target_dir . parent () . unwrap ()) . context ("failed to create target directory") ? ; fs :: rename (staging_dir . path () , sysroot_target_dir) . context ("failed installing sysroot") ? ; Ok (SysrootStatus :: SysrootBuilt) } }
};
}
