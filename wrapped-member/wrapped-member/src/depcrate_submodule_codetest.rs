// Generated macro for test (module)
macro_rules! Depcrate_submodule_codetest {
() => {
// Module: crate::submodule::code
// Provides: {"test"}
// Dependencies: {}
mod test { proptest ! { # [test] fn the_test (_ in 0u32 .. 100) { panic ! () } } }
};
}
