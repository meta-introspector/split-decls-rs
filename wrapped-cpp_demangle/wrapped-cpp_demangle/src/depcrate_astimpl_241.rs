// Generated macro for impl_241 (impl)
macro_rules! Depcrate_astimpl_241 {
() => {
// Module: crate::ast
// Provides: {"impl_241"}
// Dependencies: {}
impl < 'a > Hash for & 'a TemplateParam { fn hash < H > (& self , state : & mut H) where H : Hasher , { let self_ref : & TemplateParam = * self ; let self_ptr = self_ref as * const TemplateParam ; self_ptr . hash (state) ; } }
};
}
