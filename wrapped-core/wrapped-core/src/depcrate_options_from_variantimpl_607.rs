// Generated macro for impl_607 (impl)
macro_rules! Depcrate_options_from_variantimpl_607 {
() => {
// Module: crate::options::from_variant
// Provides: {"impl_607"}
// Dependencies: {}
impl ParseAttribute for FromVariantOptions { fn parse_nested (& mut self , mi : & Meta) -> Result < () > { if mi . path () . is_ident ("supports") { self . supports = FromMeta :: from_meta (mi) ? ; Ok (()) } else { self . base . parse_nested (mi) } } }
};
}
