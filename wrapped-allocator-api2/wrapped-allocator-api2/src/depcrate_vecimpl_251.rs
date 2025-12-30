// Generated macro for impl_251 (impl)
macro_rules! Depcrate_vecimpl_251 {
() => {
// Module: crate::vec
// Provides: {"impl_251"}
// Dependencies: {}
impl < T , I : SliceIndex < [T] > , A : Allocator > IndexMut < I > for Vec < T , A > { # [inline (always)] fn index_mut (& mut self , index : I) -> & mut Self :: Output { IndexMut :: index_mut (& mut * * self , index) } }
};
}
