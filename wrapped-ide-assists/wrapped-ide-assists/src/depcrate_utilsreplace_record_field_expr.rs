// Generated macro for replace_record_field_expr (function)
macro_rules! Depcrate_utilsreplace_record_field_expr {
() => {
// Module: crate::utils
// Provides: {"replace_record_field_expr"}
// Dependencies: {}
# [doc = " Replaces the record expression, handling field shorthands including inside macros."] pub (crate) fn replace_record_field_expr (ctx : & AssistContext < '_ > , edit : & mut SourceChangeBuilder , record_field : ast :: RecordExprField , initializer : ast :: Expr ,) { if let Some (ast :: Expr :: PathExpr (path_expr)) = record_field . expr () { let file_range = ctx . sema . original_range (path_expr . syntax ()) ; edit . insert (file_range . range . end () , format ! (": {}" , initializer . syntax () . text ())) } else if let Some (expr) = record_field . expr () { let file_range = ctx . sema . original_range (expr . syntax ()) ; edit . replace (file_range . range , initializer . syntax () . text ()) ; } }
};
}
