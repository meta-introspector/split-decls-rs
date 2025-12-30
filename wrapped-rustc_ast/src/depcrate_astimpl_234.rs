// Generated macro for impl_234 (impl)
macro_rules! Depcrate_astimpl_234 {
() => {
// Module: crate::ast
// Provides: {"impl_234"}
// Dependencies: {}
impl FnHeader { # [doc = " Does this function header have any qualifiers or is it empty?"] pub fn has_qualifiers (& self) -> bool { let Self { safety , coroutine_kind , constness , ext } = self ; matches ! (safety , Safety :: Unsafe (_)) || coroutine_kind . is_some () || matches ! (constness , Const :: Yes (_)) || ! matches ! (ext , Extern :: None) } }
};
}
