// Generated macro for impl_1176 (impl)
macro_rules! Depcrate_type_impl_1176 {
() => {
// Module: crate::type_
// Provides: {"impl_1176"}
// Dependencies: {}
impl < 'll , CX : Borrow < SCx < 'll > > > GenericCx < 'll , CX > { pub (crate) fn llcx (& self) -> & 'll llvm :: Context { (* * self) . borrow () . llcx } pub (crate) fn llmod (& self) -> & 'll llvm :: Module { (* * self) . borrow () . llmod } pub (crate) fn isize_ty (& self) -> & 'll Type { (* * self) . borrow () . isize_ty } pub (crate) fn type_variadic_func (& self , args : & [& 'll Type] , ret : & 'll Type) -> & 'll Type { unsafe { llvm :: LLVMFunctionType (ret , args . as_ptr () , args . len () as c_uint , TRUE) } } pub (crate) fn type_i1 (& self) -> & 'll Type { unsafe { llvm :: LLVMInt1TypeInContext (self . llcx ()) } } pub (crate) fn type_struct (& self , els : & [& 'll Type] , packed : bool) -> & 'll Type { unsafe { llvm :: LLVMStructTypeInContext (self . llcx () , els . as_ptr () , els . len () as c_uint , packed . to_llvm_bool () ,) } } }
};
}
