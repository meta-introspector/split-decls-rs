// Generated macro for check_directive (function)
macro_rules! Depcrate_directivescheck_directive {
() => {
// Module: crate::directives
// Provides: {"check_directive"}
// Dependencies: {}
pub (crate) fn check_directive < 'a > (directive_ln : & 'a str , mode : TestMode ,) -> CheckDirectiveResult < 'a > { let (directive_name , post) = directive_ln . split_once ([':' , ' ']) . unwrap_or ((directive_ln , "")) ; let is_known_directive = KNOWN_DIRECTIVE_NAMES . contains (& directive_name) || match mode { TestMode :: Rustdoc => KNOWN_HTMLDOCCK_DIRECTIVE_NAMES . contains (& directive_name) , TestMode :: RustdocJson => KNOWN_JSONDOCCK_DIRECTIVE_NAMES . contains (& directive_name) , _ => false , } ; let trailing = post . trim () . split_once (' ') . map (| (pre , _) | pre) . unwrap_or (post) ; let trailing_directive = { directive_ln . get (directive_name . len () ..) . is_some_and (| s | s . starts_with (' ')) && KNOWN_DIRECTIVE_NAMES . contains (& trailing) } . then_some (trailing) ; CheckDirectiveResult { is_known_directive , trailing_directive } }
};
}
