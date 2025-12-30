// Generated macro for tests (module)
macro_rules! Depcrate_matcher_support_auto_eqtests {
() => {
// Module: crate::matcher_support::auto_eq
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: prelude :: * ; use crate :: Result ; # [test] fn auto_ref_matcher () -> Result < () > { verify_that ! (123 , __auto_eq ! (ge (9))) } # [test] fn auto_ref_expected () -> Result < () > { verify_that ! (123 , __auto_eq ! (123)) } # [test] fn auto_ref_on_ref_matcher () -> Result < () > { let matcher = eq (123) ; verify_that ! (123 , __auto_eq ! (& matcher)) } }
};
}
