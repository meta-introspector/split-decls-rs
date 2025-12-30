// Generated macro for impl_653 (impl)
macro_rules! Depcrate_base_array_storageimpl_653 {
() => {
// Module: crate::base::array_storage
// Provides: {"impl_653"}
// Dependencies: {}
# [cfg (feature = "serde-serialize-no-std")] impl < T , const R : usize , const C : usize > ArrayStorageVisitor < T , R , C > where T : Scalar , { # [doc = " Construct a new sequence visitor."] pub fn new () -> Self { ArrayStorageVisitor { marker : PhantomData , } } }
};
}
