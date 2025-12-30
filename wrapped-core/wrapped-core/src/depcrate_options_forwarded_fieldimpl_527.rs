// Generated macro for impl_527 (impl)
macro_rules! Depcrate_options_forwarded_fieldimpl_527 {
() => {
// Module: crate::options::forwarded_field
// Provides: {"impl_527"}
// Dependencies: {}
impl ParseAttribute for ForwardedField { fn parse_nested (& mut self , mi : & syn :: Meta) -> crate :: Result < () > { if mi . path () . is_ident ("with") { if self . with . is_some () { return Err (Error :: duplicate_field_path (mi . path ()) . with_span (mi)) ; } self . with = FromMeta :: from_meta (mi) ? ; Ok (()) } else { Err (Error :: unknown_field_path_with_alts (mi . path () , & ["with"]) . with_span (mi)) } } }
};
}
