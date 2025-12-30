// Generated macro for name_partial (function)
macro_rules! Depcrate_referencename_partial {
() => {
// Module: crate::reference
// Provides: {"name_partial"}
// Dependencies: {}
# [doc = " Validate a partial reference name. As it is assumed to be partial, names like `some-name` is allowed"] # [doc = " even though these would be disallowed with when using [`name()`]."] pub fn name_partial (path : & BStr) -> Result < & BStr , name :: Error > { match validate (path , Mode :: Partial) ? { None => Ok (path) , Some (_) => { unreachable ! ("Without sanitization, there is no chance a sanitized version is returned.") } } }
};
}
