// Generated macro for impl_9038 (impl)
macro_rules! Depcrate_ptr_ptr_argimpl_9038 {
() => {
// Module: crate::ptr::ptr_arg
// Provides: {"impl_9038"}
// Dependencies: {}
impl PtrArg < '_ > { fn build_msg (& self) -> String { format ! ("writing `&{}{}` instead of `&{}{}` involves a new object where a slice will do" , self . ref_prefix . mutability . prefix_str () , self . ty_name , self . ref_prefix . mutability . prefix_str () , self . deref_ty . argless_str () ,) } fn mutability (& self) -> Mutability { self . ref_prefix . mutability } }
};
}
