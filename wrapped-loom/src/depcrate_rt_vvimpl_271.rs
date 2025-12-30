// Generated macro for impl_271 (impl)
macro_rules! Depcrate_rt_vvimpl_271 {
() => {
// Module: crate::rt::vv
// Provides: {"impl_271"}
// Dependencies: {}
impl cmp :: PartialOrd for VersionVec { fn partial_cmp (& self , other : & VersionVec) -> Option < cmp :: Ordering > { use cmp :: Ordering :: * ; let mut ret = Equal ; for i in 0 .. MAX_THREADS { let a = self . versions [i] ; let b = other . versions [i] ; match a . cmp (& b) { Equal => { } Less if ret == Greater => return None , Greater if ret == Less => return None , ordering => ret = ordering , } } Some (ret) } }
};
}
