// Generated macro for is_not_whitespace (function)
macro_rules! Depcrate_cfg_processis_not_whitespace {
() => {
// Module: crate::cfg_process
// Provides: {"is_not_whitespace"}
// Dependencies: {}
fn is_not_whitespace (element : & NodeOrToken < ast :: TokenTree , syntax :: SyntaxToken >) -> bool { ! matches ! (element , NodeOrToken :: Token (token) if (token . kind () == SyntaxKind :: WHITESPACE)) }
};
}
