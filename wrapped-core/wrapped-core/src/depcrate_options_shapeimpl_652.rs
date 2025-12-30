// Generated macro for impl_652 (impl)
macro_rules! Depcrate_options_shapeimpl_652 {
() => {
// Module: crate::options::shape
// Provides: {"impl_652"}
// Dependencies: {}
impl FromMeta for DeriveInputShapeSet { fn from_list (items : & [NestedMeta]) -> Result < Self > { let mut new = DeriveInputShapeSet :: default () ; for item in items { if let NestedMeta :: Meta (Meta :: Path (ref path)) = * item { let ident = & path . segments . first () . unwrap () . ident ; let word = ident . to_string () ; if word == "any" { new . any = true ; } else if word . starts_with ("enum_") { new . enum_values . set_word (& word) . map_err (| e | e . with_span (& ident)) ? ; } else if word . starts_with ("struct_") { new . struct_values . set_word (& word) . map_err (| e | e . with_span (& ident)) ? ; } else { return Err (Error :: unknown_value (& word) . with_span (& ident)) ; } } else { return Err (Error :: unsupported_format ("non-word") . with_span (item)) ; } } Ok (new) } }
};
}
