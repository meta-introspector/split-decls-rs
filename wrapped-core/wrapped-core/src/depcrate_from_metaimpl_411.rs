// Generated macro for impl_411 (impl)
macro_rules! Depcrate_from_metaimpl_411 {
() => {
// Module: crate::from_meta
// Provides: {"impl_411"}
// Dependencies: {}
# [doc = " Parsing support for punctuated. This attempts to preserve span information"] # [doc = " when available, but also supports parsing strings with the call site as the"] # [doc = " emitted span."] impl < T : syn :: parse :: Parse , P : syn :: parse :: Parse > FromMeta for syn :: punctuated :: Punctuated < T , P > { fn from_value (value : & Lit) -> Result < Self > { if let Lit :: Str (ref ident) = * value { ident . parse_with (syn :: punctuated :: Punctuated :: parse_terminated) . map_err (| _ | Error :: unknown_lit_str_value (ident)) } else { Err (Error :: unexpected_lit_type (value)) } } }
};
}
