// Generated macro for impl_550 (impl)
macro_rules! Depcrate_options_from_deriveimpl_550 {
() => {
// Module: crate::options::from_derive
// Provides: {"impl_550"}
// Dependencies: {}
impl ParseAttribute for FdiOptions { fn parse_nested (& mut self , mi : & syn :: Meta) -> Result < () > { if mi . path () . is_ident ("supports") { self . supports = FromMeta :: from_meta (mi) ? ; Ok (()) } else { self . base . parse_nested (mi) } } }
};
}
