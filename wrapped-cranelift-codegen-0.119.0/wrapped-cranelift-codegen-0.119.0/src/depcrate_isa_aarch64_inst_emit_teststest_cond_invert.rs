// Generated macro for test_cond_invert (function)
macro_rules! Depcrate_isa_aarch64_inst_emit_teststest_cond_invert {
() => {
// Module: crate::isa::aarch64::inst::emit_tests
// Provides: {"test_cond_invert"}
// Dependencies: {}
# [test] fn test_cond_invert () { for cond in vec ! [Cond :: Eq , Cond :: Ne , Cond :: Hs , Cond :: Lo , Cond :: Mi , Cond :: Pl , Cond :: Vs , Cond :: Vc , Cond :: Hi , Cond :: Ls , Cond :: Ge , Cond :: Lt , Cond :: Gt , Cond :: Le , Cond :: Al , Cond :: Nv ,] . into_iter () { assert_eq ! (cond . invert () . invert () , cond) ; } }
};
}
