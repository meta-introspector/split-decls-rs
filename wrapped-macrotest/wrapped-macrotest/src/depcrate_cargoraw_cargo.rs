// Generated macro for raw_cargo (function)
macro_rules! Depcrate_cargoraw_cargo {
() => {
// Module: crate::cargo
// Provides: {"raw_cargo"}
// Dependencies: {}
fn raw_cargo () -> Command { Command :: new (option_env ! ("CARGO") . unwrap_or ("cargo")) }
};
}
