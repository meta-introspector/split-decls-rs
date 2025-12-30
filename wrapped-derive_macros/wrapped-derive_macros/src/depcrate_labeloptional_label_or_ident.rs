// Generated macro for optional_label_or_ident (function)
macro_rules! Depcrate_labeloptional_label_or_ident {
() => {
// Module: crate::label
// Provides: {"optional_label_or_ident"}
// Dependencies: {}
pub (crate) fn optional_label_or_ident < 'a > (explicit : Option < LabelValue > , ident : Option < & Ident > ,) -> Option < Label > { explicit . map (explicit_label) . or_else (| | ident . map (ident_label)) }
};
}
