// Generated macro for test (module)
macro_rules! Depcrate_distr_slicetest {
() => {
// Module: crate::distr::slice
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use core :: iter ; # [test] fn value_stability () { let rng = crate :: test :: rng (651) ; let slice = Choose :: new (b"escaped emus explore extensively") . unwrap () ; let expected = b"eaxee" ; assert ! (iter :: zip (slice . sample_iter (rng) , expected) . all (| (a , b) | a == b)) ; } }
};
}
