// Generated macro for impl_381 (impl)
macro_rules! Depcrate_builderimpl_381 {
() => {
// Module: crate::builder
// Provides: {"impl_381"}
// Dependencies: {}
impl < 'a , 'll , CX : Borrow < SCx < 'll > > > Drop for GenericBuilder < 'a , 'll , CX > { fn drop (& mut self) { unsafe { llvm :: LLVMDisposeBuilder (& mut * (self . llbuilder as * mut _)) ; } } }
};
}
