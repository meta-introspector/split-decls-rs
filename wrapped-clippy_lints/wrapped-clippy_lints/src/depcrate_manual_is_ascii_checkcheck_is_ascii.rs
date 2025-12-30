// Generated macro for check_is_ascii (function)
macro_rules! Depcrate_manual_is_ascii_checkcheck_is_ascii {
() => {
// Module: crate::manual_is_ascii_check
// Provides: {"check_is_ascii"}
// Dependencies: {}
fn check_is_ascii (cx : & LateContext < '_ > , span : Span , recv : & Expr < '_ > , range : & CharRange , ty_sugg : Option < (Span , Ty < '_ >) > ,) { let sugg = match range { CharRange :: UpperChar => "is_ascii_uppercase" , CharRange :: LowerChar => "is_ascii_lowercase" , CharRange :: FullChar => "is_ascii_alphabetic" , CharRange :: Digit => "is_ascii_digit" , CharRange :: HexDigit => "is_ascii_hexdigit" , CharRange :: Otherwise | CharRange :: LowerHexLetter | CharRange :: UpperHexLetter => return , } ; let mut app = Applicability :: MachineApplicable ; let recv = Sugg :: hir_with_context (cx , recv , span . ctxt () , "_" , & mut app) . maybe_paren () ; let mut suggestion = vec ! [(span , format ! ("{recv}.{sugg}()"))] ; if let Some ((ty_span , ty)) = ty_sugg { suggestion . push ((ty_span , format ! ("{recv}: {ty}"))) ; } span_lint_and_then (cx , MANUAL_IS_ASCII_CHECK , span , "manual check for common ascii range" , | diag | { diag . multipart_suggestion ("try" , suggestion , app) ; } ,) ; }
};
}
