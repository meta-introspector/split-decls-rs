// Generated macro for tests (module)
macro_rules! Depcrate_isa_aarch64_insttests {
() => {
// Module: crate::isa::aarch64::inst
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn inst_size_test () { let expected = if cfg ! (target_pointer_width = "32") && ! cfg ! (target_arch = "arm") { 28 } else { 32 } ; assert_eq ! (expected , std :: mem :: size_of ::< Inst > ()) ; } }
};
}
