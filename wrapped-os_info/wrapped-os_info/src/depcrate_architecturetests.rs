// Generated macro for tests (module)
macro_rules! Depcrate_architecturetests {
() => {
// Module: crate::architecture
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn uname_nonempty () { let val = get () . expect ("architecture::get() failed") ; assert ! (! val . is_empty ()) ; } }
};
}
