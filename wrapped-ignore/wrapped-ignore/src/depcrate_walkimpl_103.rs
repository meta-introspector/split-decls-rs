// Generated macro for impl_103 (impl)
macro_rules! Depcrate_walkimpl_103 {
() => {
// Module: crate::walk
// Provides: {"impl_103"}
// Dependencies: {}
impl From < WalkDir > for WalkEventIter { fn from (it : WalkDir) -> WalkEventIter { WalkEventIter { depth : 0 , it : it . into_iter () , next : None } } }
};
}
