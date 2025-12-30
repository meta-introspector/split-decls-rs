// Generated macro for has_rustup_stable (function)
macro_rules! Depcratehas_rustup_stable {
() => {
// Module: crate
// Provides: {"has_rustup_stable"}
// Dependencies: {}
fn has_rustup_stable () -> bool { if option_env ! ("CARGO_TEST_DISABLE_NIGHTLY") . is_some () { return false ; } let home = match option_env ! ("CARGO_HOME") { Some (home) => home , None if is_ci () => panic ! ("expected to run under rustup") , None => return false , } ; let cargo = Path :: new (home) . join ("bin/cargo") ; check_command (& cargo , & ["+stable" , "--version"]) }
};
}
