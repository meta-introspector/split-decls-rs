// Generated macro for is_incomplete_range (function)
macro_rules! Depcrate_almost_complete_rangeis_incomplete_range {
() => {
// Module: crate::almost_complete_range
// Provides: {"is_incomplete_range"}
// Dependencies: {}
fn is_incomplete_range (start : & Expr , end : & Expr) -> bool { match (& start . peel_parens () . kind , & end . peel_parens () . kind) { (& ExprKind :: Lit (start_lit) , & ExprKind :: Lit (end_lit)) => { matches ! ((LitKind :: from_token_lit (start_lit) , LitKind :: from_token_lit (end_lit) ,) , (Ok (LitKind :: Byte (b'a') | LitKind :: Char ('a')) , Ok (LitKind :: Byte (b'z') | LitKind :: Char ('z'))) | (Ok (LitKind :: Byte (b'A') | LitKind :: Char ('A')) , Ok (LitKind :: Byte (b'Z') | LitKind :: Char ('Z')) ,) | (Ok (LitKind :: Byte (b'0') | LitKind :: Char ('0')) , Ok (LitKind :: Byte (b'9') | LitKind :: Char ('9')) ,)) } , _ => false , } }
};
}
