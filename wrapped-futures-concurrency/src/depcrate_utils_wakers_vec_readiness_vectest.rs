// Generated macro for test (module)
macro_rules! Depcrate_utils_wakers_vec_readiness_vectest {
() => {
// Module: crate::utils::wakers::vec::readiness_vec
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn resize () { let mut readiness = ReadinessVec :: new (10) ; assert ! (readiness . any_ready ()) ; readiness . clear_all_ready () ; assert ! (! readiness . any_ready ()) ; readiness . set_ready (9) ; assert ! (readiness . any_ready ()) ; readiness . resize (9) ; assert ! (! readiness . any_ready ()) ; readiness . resize (10) ; assert ! (readiness . any_ready ()) ; } }
};
}
