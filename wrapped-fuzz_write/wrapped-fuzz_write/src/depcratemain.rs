// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { fuzz ! (| data : & [u8] | { if let Ok (test_case) = Unstructured :: new (data) . arbitrary ::< FuzzTestCase > () { test_case . execute (& mut NoopWrite :: default () , true) . unwrap () ; } }) ; }
};
}
