// Generated macro for Reallocator (trait)
macro_rules! Depcrate_base_allocatorReallocator {
() => {
// Module: crate::base::allocator
// Provides: {"Reallocator"}
// Dependencies: {}
# [doc = " A matrix reallocator. Changes the size of the memory buffer that initially contains (`RFrom` ×"] # [doc = " `CFrom`) elements to a smaller or larger size (`RTo`, `CTo`)."] pub trait Reallocator < T : Scalar , RFrom : Dim , CFrom : Dim , RTo : Dim , CTo : Dim > : Allocator < RFrom , CFrom > + Allocator < RTo , CTo > { # [doc = " Reallocates a buffer of shape `(RTo, CTo)`, possibly reusing a previously allocated buffer"] # [doc = " `buf`. Data stored by `buf` are linearly copied to the output:"] # [doc = ""] # [doc = " # Safety"] # [doc = " The following invariants must be respected by the implementors of this method:"] # [doc = " * The copy is performed as if both were just arrays (without taking into account the matrix structure)."] # [doc = " * If the underlying buffer is being shrunk, the removed elements must **not** be dropped"] # [doc = "   by this method. Dropping them is the responsibility of the caller."] unsafe fn reallocate_copy (nrows : RTo , ncols : CTo , buf : < Self as Allocator < RFrom , CFrom > > :: Buffer < T > ,) -> < Self as Allocator < RTo , CTo > > :: BufferUninit < T > ; }
};
}
