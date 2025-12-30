// Generated macro for impl_354 (impl)
macro_rules! Depcrate_groupbylazyimpl_354 {
() => {
// Module: crate::groupbylazy
// Provides: {"impl_354"}
// Dependencies: {}
impl < A > KeyFunction < A > for ChunkIndex { type Key = usize ; # [inline (always)] fn call_mut (& mut self , _arg : A) -> Self :: Key { if self . index == self . size { self . key += 1 ; self . index = 0 ; } self . index += 1 ; self . key } }
};
}
