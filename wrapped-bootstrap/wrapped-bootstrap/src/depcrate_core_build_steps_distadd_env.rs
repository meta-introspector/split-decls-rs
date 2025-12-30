// Generated macro for add_env (function)
macro_rules! Depcrate_core_build_steps_distadd_env {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"add_env"}
// Dependencies: {}
fn add_env (builder : & Builder < '_ > , cmd : & mut BootstrapCommand , target : TargetSelection , built_tools : & HashSet < & 'static str > ,) { let mut parts = builder . version . split ('.') ; cmd . env ("CFG_RELEASE_INFO" , builder . rust_version ()) . env ("CFG_RELEASE_NUM" , & builder . version) . env ("CFG_RELEASE" , builder . rust_release ()) . env ("CFG_VER_MAJOR" , parts . next () . unwrap ()) . env ("CFG_VER_MINOR" , parts . next () . unwrap ()) . env ("CFG_VER_PATCH" , parts . next () . unwrap ()) . env ("CFG_VER_BUILD" , "0") . env ("CFG_PACKAGE_VERS" , builder . rust_package_vers ()) . env ("CFG_PACKAGE_NAME" , pkgname (builder , "rust")) . env ("CFG_BUILD" , target . triple) . env ("CFG_CHANNEL" , & builder . config . channel) ; if target . contains ("windows-gnullvm") { cmd . env ("CFG_MINGW" , "1") . env ("CFG_ABI" , "LLVM") ; } else if target . is_windows_gnu () { cmd . env ("CFG_MINGW" , "1") . env ("CFG_ABI" , "GNU") ; } else { cmd . env ("CFG_MINGW" , "0") . env ("CFG_ABI" , "MSVC") ; } let mut define_optional_tool = | tool_name : & str , env_name : & str | { cmd . env (env_name , if built_tools . contains (tool_name) { "1" } else { "0" }) ; } ; define_optional_tool ("rustfmt" , "CFG_RUSTFMT") ; define_optional_tool ("clippy" , "CFG_CLIPPY") ; define_optional_tool ("miri" , "CFG_MIRI") ; define_optional_tool ("rust-analyzer" , "CFG_RA") ; }
};
}
