// Generated macro for intersperse_with (function)
macro_rules! Depcrate_intersperseintersperse_with {
() => {
// Module: crate::intersperse
// Provides: {"intersperse_with"}
// Dependencies: {}
# [doc = " Create a new `IntersperseWith` iterator"] pub fn intersperse_with < I , ElemF > (iter : I , elt : ElemF) -> IntersperseWith < I , ElemF > where I : Iterator , { IntersperseWith { peek : None , iter : iter . fuse () , element : elt , } }
};
}
