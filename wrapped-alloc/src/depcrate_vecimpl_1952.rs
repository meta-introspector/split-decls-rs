// Generated macro for impl_1952 (impl)
macro_rules! Depcrate_vecimpl_1952 {
() => {
// Module: crate::vec
// Provides: {"impl_1952"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , I : SliceIndex < [T] > , A : Allocator > IndexMut < I > for Vec < T , A > { # [inline] fn index_mut (& mut self , index : I) -> & mut Self :: Output { IndexMut :: index_mut (& mut * * self , index) } }
};
}
