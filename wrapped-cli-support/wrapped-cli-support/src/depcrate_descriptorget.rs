// Generated macro for get (function)
macro_rules! Depcrate_descriptorget {
() => {
// Module: crate::descriptor
// Provides: {"get"}
// Dependencies: {}
fn get (a : & mut & [u32]) -> u32 { let ret = a [0] ; * a = & a [1 ..] ; ret }
};
}
