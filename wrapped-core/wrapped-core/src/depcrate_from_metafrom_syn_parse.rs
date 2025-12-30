// Generated macro for from_syn_parse (macro)
macro_rules! Depcrate_from_metafrom_syn_parse {
() => {
// Module: crate::from_meta
// Provides: {"from_syn_parse"}
// Dependencies: {}
# [doc = " Adapter from `syn::parse::Parse` to `FromMeta` for items that cannot"] # [doc = " be expressed in a [`syn::MetaNameValue`]."] # [doc = ""] # [doc = " This cannot be a blanket impl, due to the `syn::Lit` family's need to handle non-string values."] # [doc = " Therefore, we use a macro and a lot of impls."] macro_rules ! from_syn_parse { ($ ty : path) => { impl FromMeta for $ ty { fn from_string (value : & str) -> Result < Self > { syn :: parse_str (value) . map_err (| _ | Error :: unknown_value (value)) } fn from_value (value : &:: syn :: Lit) -> Result < Self > { if let :: syn :: Lit :: Str (ref v) = * value { v . parse ::<$ ty > () . map_err (| _ | Error :: unknown_lit_str_value (v)) } else { Err (Error :: unexpected_lit_type (value)) } } } } ; }
};
}
