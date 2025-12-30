// Generated macro for subty_if_name (function)
macro_rules! Depcrate_utils_tysubty_if_name {
() => {
// Module: crate::utils::ty
// Provides: {"subty_if_name"}
// Dependencies: {}
pub (crate) fn subty_if_name < 'a > (ty : & 'a Type , name : & str) -> Option < & 'a Type > { subty_if (ty , | seg | seg . ident == name) }
};
}
