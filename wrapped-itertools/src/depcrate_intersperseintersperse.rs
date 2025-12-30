// Generated macro for intersperse (function)
macro_rules! Depcrate_intersperseintersperse {
() => {
// Module: crate::intersperse
// Provides: {"intersperse"}
// Dependencies: {}
# [doc = " Create a new Intersperse iterator"] pub fn intersperse < I > (iter : I , elt : I :: Item) -> Intersperse < I > where I : Iterator , { intersperse_with (iter , IntersperseElementSimple (elt)) }
};
}
