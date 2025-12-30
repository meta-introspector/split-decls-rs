// Generated macro for impl_579 (impl)
macro_rules! Depcrate_options_from_metaimpl_579 {
() => {
// Module: crate::options::from_meta
// Provides: {"impl_579"}
// Dependencies: {}
impl FromMetaOptions { pub fn new (di : & syn :: DeriveInput) -> Result < Self > { (FromMetaOptions { base : Core :: start (di) ? , from_word : None , from_none : None , from_expr : None , derive_syn_parse : None , }) . parse_attributes (& di . attrs) ? . parse_body (& di . data) } # [doc = " Get the `from_word` method body, if one exists. This can come from direct use of"] # [doc = " `#[darling(from_word = ...)]` on the container or from use of `#[darling(word)]` on"] # [doc = " a unit variant."] # [allow (clippy :: wrong_self_convention ,)] fn from_word (& self) -> Option < Cow < '_ , Callable > > { self . from_word . as_ref () . map (Cow :: Borrowed) . or_else (| | { if let Data :: Enum (ref variants) = self . base . data { let variant = variants . iter () . find (| v | v . word . map (| x | * x) . unwrap_or_default ()) ? ; let variant_ident = & variant . ident ; let closure : syn :: ExprClosure = parse_quote ! { || :: darling :: export :: Ok (Self ::# variant_ident) } ; Some (Cow :: Owned (Callable :: from (closure))) } else { None } }) } }
};
}
