// Generated macro for FUNCTION_REPLACEMENTS (static)
macro_rules! Depcrate_non_std_lazy_staticsFUNCTION_REPLACEMENTS {
() => {
// Module: crate::non_std_lazy_statics
// Provides: {"FUNCTION_REPLACEMENTS"}
// Dependencies: {}
# [doc = " A list containing functions with corresponding replacements in `LazyLock`."] # [doc = ""] # [doc = " Some functions could be replaced as well if we have replaced `Lazy` to `LazyLock`,"] # [doc = " therefore after suggesting replace the type, we need to make sure the function calls can be"] # [doc = " replaced, otherwise the suggestions cannot be applied thus the applicability should be"] # [doc = " [`Applicability::Unspecified`] or [`Applicability::MaybeIncorrect`]."] static FUNCTION_REPLACEMENTS : & [(& str , Option < & str >)] = & [("once_cell::sync::Lazy::force" , Some ("std::sync::LazyLock::force")) , ("once_cell::sync::Lazy::get" , None) , ("once_cell::sync::Lazy::new" , Some ("std::sync::LazyLock::new")) ,] ;
};
}
