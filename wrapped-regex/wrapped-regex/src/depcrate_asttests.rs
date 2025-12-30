// Generated macro for tests (module)
macro_rules! Depcrate_asttests {
() => {
// Module: crate::ast
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; fn analysis (pattern : & str) -> AstAnalysis { AstAnalysis :: from_pattern (pattern) . unwrap () } # [test] fn various () { let x = analysis ("") ; assert ! (! x . any_uppercase) ; assert ! (! x . any_literal) ; let x = analysis ("foo") ; assert ! (! x . any_uppercase) ; assert ! (x . any_literal) ; let x = analysis ("Foo") ; assert ! (x . any_uppercase) ; assert ! (x . any_literal) ; let x = analysis ("foO") ; assert ! (x . any_uppercase) ; assert ! (x . any_literal) ; let x = analysis (r"foo\\") ; assert ! (! x . any_uppercase) ; assert ! (x . any_literal) ; let x = analysis (r"foo\w") ; assert ! (! x . any_uppercase) ; assert ! (x . any_literal) ; let x = analysis (r"foo\S") ; assert ! (! x . any_uppercase) ; assert ! (x . any_literal) ; let x = analysis (r"foo\p{Ll}") ; assert ! (! x . any_uppercase) ; assert ! (x . any_literal) ; let x = analysis (r"foo[a-z]") ; assert ! (! x . any_uppercase) ; assert ! (x . any_literal) ; let x = analysis (r"foo[A-Z]") ; assert ! (x . any_uppercase) ; assert ! (x . any_literal) ; let x = analysis (r"foo[\S\t]") ; assert ! (! x . any_uppercase) ; assert ! (x . any_literal) ; let x = analysis (r"foo\\S") ; assert ! (x . any_uppercase) ; assert ! (x . any_literal) ; let x = analysis (r"\p{Ll}") ; assert ! (! x . any_uppercase) ; assert ! (! x . any_literal) ; let x = analysis (r"aBc\w") ; assert ! (x . any_uppercase) ; assert ! (x . any_literal) ; let x = analysis (r"a\u0061") ; assert ! (! x . any_uppercase) ; assert ! (x . any_literal) ; } }
};
}
