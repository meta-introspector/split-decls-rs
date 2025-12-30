// Generated macro for tests (module)
macro_rules! Depcrate_parsingtests {
() => {
// Module: crate::parsing
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn parser_happy_case () { fn token (kind : SyntaxKind , text : & str) -> PatternElement { PatternElement :: Token (Token { kind , text : SmolStr :: new (text) }) } fn placeholder (name : & str) -> PatternElement { PatternElement :: Placeholder (Placeholder :: new (SmolStr :: new (name) , Vec :: new ())) } let result : SsrRule = "foo($a, $b) ==>> bar($b, $a)" . parse () . unwrap () ; assert_eq ! (result . pattern . tokens , vec ! [token (SyntaxKind :: IDENT , "foo") , token (T ! ['('] , "(") , placeholder ("a") , token (T ! [,] , ",") , token (SyntaxKind :: WHITESPACE , " ") , placeholder ("b") , token (T ! [')'] , ")") ,]) ; assert_eq ! (result . template . tokens , vec ! [token (SyntaxKind :: IDENT , "bar") , token (T ! ['('] , "(") , placeholder ("b") , token (T ! [,] , ",") , token (SyntaxKind :: WHITESPACE , " ") , placeholder ("a") , token (T ! [')'] , ")") ,]) ; } }
};
}
