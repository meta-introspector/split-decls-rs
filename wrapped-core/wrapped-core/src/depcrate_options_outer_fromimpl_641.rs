// Generated macro for impl_641 (impl)
macro_rules! Depcrate_options_outer_fromimpl_641 {
() => {
// Module: crate::options::outer_from
// Provides: {"impl_641"}
// Dependencies: {}
impl ParseAttribute for OuterFrom { fn parse_nested (& mut self , mi : & Meta) -> Result < () > { let path = mi . path () ; if path . is_ident ("attributes") { self . attr_names = FromMeta :: from_meta (mi) ? ; } else if path . is_ident ("forward_attrs") { self . forward_attrs = FromMeta :: from_meta (mi) ? ; } else if path . is_ident ("from_ident") { self . container . default = Some (DefaultExpression :: Trait { span : path . span () , }) ; self . from_ident = true ; } else { return self . container . parse_nested (mi) ; } Ok (()) } }
};
}
