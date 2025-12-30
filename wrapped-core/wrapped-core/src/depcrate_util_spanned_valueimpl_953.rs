// Generated macro for impl_953 (impl)
macro_rules! Depcrate_util_spanned_valueimpl_953 {
() => {
// Module: crate::util::spanned_value
// Provides: {"impl_953"}
// Dependencies: {}
impl < T : FromMeta > FromMeta for SpannedValue < T > { fn from_meta (item : & syn :: Meta) -> Result < Self > { let value = T :: from_meta (item) . map_err (| e | e . with_span (item)) ? ; let span = match item { syn :: Meta :: Path (path) => path . span () , syn :: Meta :: List (list) => list . tokens . span () , syn :: Meta :: NameValue (nv) => nv . value . span () , } ; Ok (Self :: new (value , span)) } fn from_nested_meta (item : & crate :: ast :: NestedMeta) -> Result < Self > { T :: from_nested_meta (item) . map (| value | Self :: new (value , item . span ())) . map_err (| e | e . with_span (item)) } fn from_value (literal : & syn :: Lit) -> Result < Self > { T :: from_value (literal) . map (| value | Self :: new (value , literal . span ())) . map_err (| e | e . with_span (literal)) } fn from_expr (expr : & syn :: Expr) -> Result < Self > { T :: from_expr (expr) . map (| value | Self :: new (value , expr . span ())) . map_err (| e | e . with_span (expr)) } }
};
}
