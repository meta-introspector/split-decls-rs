// Generated macro for report_unexpected_meta_item_lit (function)
macro_rules! Depcrate_derivereport_unexpected_meta_item_lit {
() => {
// Module: crate::derive
// Provides: {"report_unexpected_meta_item_lit"}
// Dependencies: {}
fn report_unexpected_meta_item_lit (sess : & Session , lit : & ast :: MetaItemLit) { let help = match lit . kind { ast :: LitKind :: Str (_ , ast :: StrStyle :: Cooked) if rustc_lexer :: is_ident (lit . symbol . as_str ()) => { errors :: BadDeriveLitHelp :: StrLit { sym : lit . symbol } } _ => errors :: BadDeriveLitHelp :: Other , } ; sess . dcx () . emit_err (errors :: BadDeriveLit { span : lit . span , help }) ; }
};
}
