// Generated macro for impl_250 (impl)
macro_rules! Depcrate_vecimpl_250 {
() => {
// Module: crate::vec
// Provides: {"impl_250"}
// Dependencies: {}
impl < T , I : SliceIndex < [T] > , A : Allocator > Index < I > for Vec < T , A > { type Output = I :: Output ; # [inline (always)] fn index (& self , index : I) -> & Self :: Output { Index :: index (& * * self , index) } }
};
}
