// Generated macro for check (function)
macro_rules! Depcrate_casts_char_lit_as_u8check {
() => {
// Module: crate::casts::char_lit_as_u8
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , expr : & Expr < '_ > , cast_from_expr : & Expr < '_ > , cast_to : Ty < '_ >) { if let ExprKind :: Lit (l) = & cast_from_expr . kind && let LitKind :: Char (c) = l . node && ty :: Uint (UintTy :: U8) == * cast_to . kind () { let mut applicability = Applicability :: MachineApplicable ; let snippet = snippet_with_applicability (cx , cast_from_expr . span , "'x'" , & mut applicability) ; span_lint_and_then (cx , CHAR_LIT_AS_U8 , expr . span , "casting a character literal to `u8` truncates" , | diag | { diag . note ("`char` is four bytes wide, but `u8` is a single byte") ; if c . is_ascii () { diag . span_suggestion (expr . span , "use a byte literal instead" , format ! ("b{snippet}") , applicability ,) ; } } ,) ; } }
};
}
