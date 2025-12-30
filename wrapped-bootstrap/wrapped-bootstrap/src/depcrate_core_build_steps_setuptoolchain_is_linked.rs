// Generated macro for toolchain_is_linked (function)
macro_rules! Depcrate_core_build_steps_setuptoolchain_is_linked {
() => {
// Module: crate::core::build_steps::setup
// Provides: {"toolchain_is_linked"}
// Dependencies: {}
fn toolchain_is_linked (builder : & Builder < '_ >) -> bool { match command ("rustup") . allow_failure () . args (["toolchain" , "list"]) . run_capture_stdout (builder) . stdout_if_ok () { Some (toolchain_list) => { if ! toolchain_list . contains ("stage1") { return false ; } println ! ("`stage1` toolchain already linked; not attempting to link `stage1` toolchain") ; } None => { println ! ("`rustup` failed to list current toolchains; not attempting to link `stage1` toolchain") ; } } true }
};
}
