// Generated macro for tests (module)
macro_rules! Depcrate_matchertests {
() => {
// Module: crate::matcher
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: prelude :: * ; use crate :: Result ; # [test] fn ref_matchers_can_be_reused () -> Result < () > { let matcher = eq (1) ; verify_that ! (1 , & matcher) ? ; verify_that ! (1 , & matcher) } # [test] fn ref_matchers_as_inner_matcher () -> Result < () > { let matcher = gt (1) ; verify_that ! ([2 , 3 , 4 , 5] , [& matcher , & matcher , & matcher , & matcher]) } }
};
}
