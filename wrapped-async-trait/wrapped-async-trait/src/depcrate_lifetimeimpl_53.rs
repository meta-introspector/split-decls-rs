// Generated macro for impl_53 (impl)
macro_rules! Depcrate_lifetimeimpl_53 {
() => {
// Module: crate::lifetime
// Provides: {"impl_53"}
// Dependencies: {}
impl CollectLifetimes { pub fn new () -> Self { CollectLifetimes { elided : Vec :: new () , explicit : Vec :: new () , } } fn visit_opt_lifetime (& mut self , reference : & Token ! [&] , lifetime : & mut Option < Lifetime >) { match lifetime { None => * lifetime = Some (self . next_lifetime (reference . span)) , Some (lifetime) => self . visit_lifetime (lifetime) , } } fn visit_lifetime (& mut self , lifetime : & mut Lifetime) { if lifetime . ident == "_" { * lifetime = self . next_lifetime (lifetime . span ()) ; } else { self . explicit . push (lifetime . clone ()) ; } } fn next_lifetime (& mut self , span : Span) -> Lifetime { let name = format ! ("'life{}" , self . elided . len ()) ; let life = Lifetime :: new (& name , span) ; self . elided . push (life . clone ()) ; life } }
};
}
