// Generated macro for tests (module)
macro_rules! Depcrate_ptrtests {
() => {
// Module: crate::ptr
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: aws_lc :: BIGNUM ; use crate :: ptr :: { DetachablePointer , ManagedPointer } ; # [test] fn test_debug () { let num = 100u64 ; let detachable_ptr : DetachablePointer < * mut BIGNUM > = DetachablePointer :: try_from (num) . unwrap () ; let debug = format ! ("{detachable_ptr:?}") ; assert ! (debug . contains ("DetachablePointer { pointer: Some(")) ; let lc_ptr = ManagedPointer :: new (detachable_ptr . detach ()) . unwrap () ; let debug = format ! ("{lc_ptr:?}") ; assert ! (debug . contains ("ManagedPointer { pointer:")) ; } }
};
}
