// Generated macro for impl_9198 (impl)
macro_rules! Depcrate_raw_stringsimpl_9198 {
() => {
// Module: crate::raw_strings
// Provides: {"impl_9198"}
// Dependencies: {}
impl EarlyLintPass for RawStrings { fn check_expr (& mut self , cx : & EarlyContext < '_ > , expr : & Expr) { if let ExprKind :: FormatArgs (format_args) = & expr . kind && ! format_args . span . in_external_macro (cx . sess () . source_map ()) && format_args . span . check_source_text (cx , | src | src . starts_with ('r')) && let Some (str) = snippet_opt (cx . sess () , format_args . span) && let count_hash = str . bytes () . skip (1) . take_while (| b | * b == b'#') . count () && let Some (str) = str . get (count_hash + 2 .. str . len () - count_hash - 1) { self . check_raw_string (cx , str , format_args . span , "r" , u8 :: try_from (count_hash) . unwrap () , "string" ,) ; } if let ExprKind :: Lit (lit) = expr . kind && let (prefix , max) = match lit . kind { LitKind :: StrRaw (max) => ("r" , max) , LitKind :: ByteStrRaw (max) => ("br" , max) , LitKind :: CStrRaw (max) => ("cr" , max) , _ => return , } && ! expr . span . in_external_macro (cx . sess () . source_map ()) && expr . span . check_source_text (cx , | src | src . starts_with (prefix)) { self . check_raw_string (cx , lit . symbol . as_str () , expr . span , prefix , max , lit . kind . descr ()) ; } } }
};
}
