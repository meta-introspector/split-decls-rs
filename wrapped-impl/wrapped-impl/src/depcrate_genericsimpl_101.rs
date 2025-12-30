// Generated macro for impl_101 (impl)
macro_rules! Depcrate_genericsimpl_101 {
() => {
// Module: crate::generics
// Provides: {"impl_101"}
// Dependencies: {}
impl < 'a > ParamsInScope < 'a > { pub fn new (generics : & 'a Generics) -> Self { ParamsInScope { names : generics . type_params () . map (| param | & param . ident) . collect () , } } pub fn intersects (& self , ty : & Type) -> bool { let mut found = false ; crawl (self , ty , & mut found) ; found } }
};
}
