// Generated macro for check (function)
macro_rules! Depcrate_write_use_debugcheck {
() => {
// Module: crate::write::use_debug
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , format_args : & FormatArgs) { for piece in & format_args . template { if let & FormatArgsPiece :: Placeholder (FormatPlaceholder { span : Some (span) , format_trait : FormatTrait :: Debug , .. }) = piece { span_lint (cx , USE_DEBUG , span , "use of `Debug`-based formatting") ; } } }
};
}
