// Generated macro for is_not_closing_paren (function)
macro_rules! Depcrate_cfg_processis_not_closing_paren {
() => {
// Module: crate::cfg_process
// Provides: {"is_not_closing_paren"}
// Dependencies: {}
fn is_not_closing_paren (element : & NodeOrToken < ast :: TokenTree , syntax :: SyntaxToken >) -> bool { ! matches ! (element , NodeOrToken :: Token (token) if (token . kind () == syntax :: T ! [')'])) }
};
}
