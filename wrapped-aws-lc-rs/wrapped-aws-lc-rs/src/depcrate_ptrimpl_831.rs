// Generated macro for impl_831 (impl)
macro_rules! Depcrate_ptrimpl_831 {
() => {
// Module: crate::ptr
// Provides: {"impl_831"}
// Dependencies: {}
impl < 'a , P : Pointer > From < & 'a ManagedPointer < P > > for ConstPointer < 'a , P :: T > { fn from (ptr : & 'a ManagedPointer < P >) -> ConstPointer < 'a , P :: T > { ConstPointer { ptr : ptr . pointer . as_const_ptr () , _lifetime : PhantomData , } } }
};
}
