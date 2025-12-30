// Generated macro for label_or_ident (function)
macro_rules! Depcrate_labellabel_or_ident {
() => {
// Module: crate::label
// Provides: {"label_or_ident"}
// Dependencies: {}
pub (crate) fn label_or_ident < 'a > (explicit : Option < LabelValue > , ident : & Ident) -> Label { explicit . map (explicit_label) . unwrap_or_else (| | ident_label (ident)) }
};
}
