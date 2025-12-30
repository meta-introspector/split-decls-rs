// Generated macro for get_cargo_nextest_command (function)
macro_rules! Depcrate_cliget_cargo_nextest_command {
() => {
// Module: crate::cli
// Provides: {"get_cargo_nextest_command"}
// Dependencies: {}
fn get_cargo_nextest_command () -> std :: process :: Command { let cargo_nextest = env :: var_os ("INSTA_CARGO_NEXTEST_BIN") ; match cargo_nextest . as_deref () { Some (cargo_nextest_bin_path) => process :: Command :: new (cargo_nextest_bin_path) , None => { let mut proc = process :: Command :: new (get_cargo ()) ; proc . arg ("nextest") ; proc } } }
};
}
