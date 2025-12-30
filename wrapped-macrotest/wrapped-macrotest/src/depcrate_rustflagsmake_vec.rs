// Generated macro for make_vec (function)
macro_rules! Depcrate_rustflagsmake_vec {
() => {
// Module: crate::rustflags
// Provides: {"make_vec"}
// Dependencies: {}
pub fn make_vec () -> Vec < String > { let mut rustflags = Vec :: new () ; for & lint in IGNORED_LINTS { rustflags . push ("-A" . to_owned ()) ; rustflags . push (lint . to_owned ()) ; } rustflags }
};
}
