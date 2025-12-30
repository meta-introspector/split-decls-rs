// Generated macro for impl_43 (impl)
macro_rules! Depcrate_gen_implimpl_43 {
() => {
// Module: crate::gen_impl
// Provides: {"impl_43"}
// Dependencies: {}
impl < A : Any , T : Any > GeneratorImpl < '_ , A , T > { # [doc = " create a new generator with default stack size"] fn init_context (& mut self) { unsafe { std :: ptr :: write (self . context . para . as_mut_ptr () , & mut self . para as & mut dyn Any ,) ; std :: ptr :: write (self . context . ret . as_mut_ptr () , & mut self . ret as & mut dyn Any) ; } } }
};
}
