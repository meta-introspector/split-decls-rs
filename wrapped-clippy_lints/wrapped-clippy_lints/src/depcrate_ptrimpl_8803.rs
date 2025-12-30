// Generated macro for impl_8803 (impl)
macro_rules! Depcrate_ptrimpl_8803 {
() => {
// Module: crate::ptr
// Provides: {"impl_8803"}
// Dependencies: {}
impl PtrArg < '_ > { fn build_msg (& self) -> String { format ! ("writing `&{}{}` instead of `&{}{}` involves a new object where a slice will do" , self . ref_prefix . mutability . prefix_str () , self . ty_name , self . ref_prefix . mutability . prefix_str () , self . deref_ty . argless_str () ,) } fn mutability (& self) -> Mutability { self . ref_prefix . mutability } }
};
}
