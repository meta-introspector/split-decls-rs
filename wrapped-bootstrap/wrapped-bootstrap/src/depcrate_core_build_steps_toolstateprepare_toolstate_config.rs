// Generated macro for prepare_toolstate_config (function)
macro_rules! Depcrate_core_build_steps_toolstateprepare_toolstate_config {
() => {
// Module: crate::core::build_steps::toolstate
// Provides: {"prepare_toolstate_config"}
// Dependencies: {}
# [doc = " Sets up config and authentication for modifying the toolstate repo."] fn prepare_toolstate_config (builder : & Builder < '_ > , token : & str) { fn git_config (builder : & Builder < '_ > , key : & str , value : & str) { helpers :: git (None) . arg ("config") . arg ("--global") . arg (key) . arg (value) . run (builder) ; } git_config (builder , "user.email" , "7378925+rust-toolstate-update@users.noreply.github.com") ; git_config (builder , "user.name" , "Rust Toolstate Update") ; git_config (builder , "credential.helper" , "store") ; let credential = format ! ("https://{token}:x-oauth-basic@github.com\n" ,) ; let git_credential_path = PathBuf :: from (t ! (env :: var ("HOME"))) . join (".git-credentials") ; t ! (fs :: write (git_credential_path , credential)) ; }
};
}
