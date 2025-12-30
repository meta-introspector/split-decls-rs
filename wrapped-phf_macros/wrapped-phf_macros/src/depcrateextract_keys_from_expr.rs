// Generated macro for extract_keys_from_expr (function)
macro_rules! Depcrateextract_keys_from_expr {
() => {
// Module: crate
// Provides: {"extract_keys_from_expr"}
// Dependencies: {}
# [doc = " Extract all keys from an expression, handling OR patterns"] fn extract_keys_from_expr (expr : & Expr) -> parse :: Result < (Vec < Expr > , Vec < ParsedKey >) > { match expr { Expr :: Binary (binary) => { if let BinOp :: BitOr (_) = binary . op { let (left_exprs , left_keys) = extract_keys_from_expr (& binary . left) ? ; let (right_exprs , right_keys) = extract_keys_from_expr (& binary . right) ? ; let mut exprs = left_exprs ; exprs . extend (right_exprs) ; let mut keys = left_keys ; keys . extend (right_keys) ; Ok ((exprs , keys)) } else { let parsed = ParsedKey :: from_expr (expr) . ok_or_else (| | Error :: new_spanned (expr , "unsupported key expression")) ? ; Ok ((vec ! [expr . clone ()] , vec ! [parsed])) } } _ => { let parsed = ParsedKey :: from_expr (expr) . ok_or_else (| | Error :: new_spanned (expr , "unsupported key expression")) ? ; Ok ((vec ! [expr . clone ()] , vec ! [parsed])) } } }
};
}
