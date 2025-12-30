// Generated macro for name (function)
macro_rules! Depcrate_tagname {
() => {
// Module: crate::tag
// Provides: {"name"}
// Dependencies: {}
# [doc = " Assure the given `input` resemble a valid git tag name, which is returned unchanged on success."] # [doc = " Tag names are provided as names, like `v1.0` or `alpha-1`, without paths."] pub fn name (input : & BStr) -> Result < & BStr , name :: Error > { match name_inner (input , Mode :: Validate) ? { None => Ok (input) , Some (_) => { unreachable ! ("When validating, the input isn't changed") } } }
};
}
