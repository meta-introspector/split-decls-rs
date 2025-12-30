// Generated macro for impl_835 (impl)
macro_rules! Depcrate_declareimpl_835 {
() => {
// Module: crate::declare
// Provides: {"impl_835"}
// Dependencies: {}
impl < 'll , CX : Borrow < SCx < 'll > > > GenericCx < 'll , CX > { # [doc = " Declare a global value."] # [doc = ""] # [doc = " If there’s a value with the same name already declared, the function will"] # [doc = " return its Value instead."] pub (crate) fn declare_global (& self , name : & str , ty : & 'll Type) -> & 'll Value { debug ! ("declare_global(name={:?})" , name) ; unsafe { llvm :: LLVMRustGetOrInsertGlobal ((* * self) . borrow () . llmod , name . as_c_char_ptr () , name . len () , ty ,) } } }
};
}
