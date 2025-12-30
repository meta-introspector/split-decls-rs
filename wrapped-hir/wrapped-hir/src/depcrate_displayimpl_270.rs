// Generated macro for impl_270 (impl)
macro_rules! Depcrate_displayimpl_270 {
() => {
// Module: crate::display
// Provides: {"impl_270"}
// Dependencies: {}
impl < 'db > HirDisplay < 'db > for SelfParam { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { let data = f . db . function_signature (self . func) ; let param = * data . params . first () . unwrap () ; match & data . store [param] { TypeRef :: Path (p) if p . is_self_type () => f . write_str ("self") , TypeRef :: Reference (ref_) if matches ! (& data . store [ref_ . ty] , TypeRef :: Path (p) if p . is_self_type ()) => { f . write_char ('&') ? ; if let Some (lifetime) = & ref_ . lifetime { lifetime . hir_fmt (f , & data . store) ? ; f . write_char (' ') ? ; } if let hir_def :: type_ref :: Mutability :: Mut = ref_ . mutability { f . write_str ("mut ") ? ; } f . write_str ("self") } _ => { f . write_str ("self: ") ? ; param . hir_fmt (f , & data . store) } } } }
};
}
