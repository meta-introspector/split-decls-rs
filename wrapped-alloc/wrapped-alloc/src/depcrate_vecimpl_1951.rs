// Generated macro for impl_1951 (impl)
macro_rules! Depcrate_vecimpl_1951 {
() => {
// Module: crate::vec
// Provides: {"impl_1951"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , I : SliceIndex < [T] > , A : Allocator > Index < I > for Vec < T , A > { type Output = I :: Output ; # [inline] fn index (& self , index : I) -> & Self :: Output { Index :: index (& * * self , index) } }
};
}
