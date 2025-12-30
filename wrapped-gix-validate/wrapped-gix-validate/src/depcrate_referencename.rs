// Generated macro for name (function)
macro_rules! Depcrate_referencename {
() => {
// Module: crate::reference
// Provides: {"name"}
// Dependencies: {}
# [doc = " Validate a reference name running all the tests in the book. This disallows lower-case references like `lower`, but also allows"] # [doc = " ones like `HEAD`, and `refs/lower`."] pub fn name (path : & BStr) -> Result < & BStr , name :: Error > { match validate (path , Mode :: Complete) ? { None => Ok (path) , Some (_) => { unreachable ! ("Without sanitization, there is no chance a sanitized version is returned.") } } }
};
}
