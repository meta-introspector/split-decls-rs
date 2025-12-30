// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl < 'a > PrintState < 'a > for State < 'a > { fn comments (& self) -> Option < & Comments < 'a > > { self . comments . as_ref () } fn comments_mut (& mut self) -> Option < & mut Comments < 'a > > { self . comments . as_mut () } fn ann_post (& mut self , ident : Ident) { self . ann . post (self , AnnNode :: Name (& ident . name)) ; } fn print_generic_args (& mut self , _ : & ast :: GenericArgs , _colons_before_params : bool) { panic ! ("AST generic args printed by HIR pretty-printer") ; } }
};
}
