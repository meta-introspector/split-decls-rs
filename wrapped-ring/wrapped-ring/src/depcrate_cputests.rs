// Generated macro for tests (module)
macro_rules! Depcrate_cputests {
() => {
// Module: crate::cpu
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_static_is_subset_of_dynamic () { let cpu = features () ; let dynamic = featureflags :: get (cpu) ; assert_eq ! (dynamic & CAPS_STATIC , CAPS_STATIC) ; } }
};
}
