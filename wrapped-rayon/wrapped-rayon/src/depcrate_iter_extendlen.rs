// Generated macro for len (function)
macro_rules! Depcrate_iter_extendlen {
() => {
// Module: crate::iter::extend
// Provides: {"len"}
// Dependencies: {}
# [doc = " Computes the total length of a `fast_collect` result."] fn len < T > (vecs : & Either < Vec < T > , LinkedList < Vec < T > > >) -> usize { match vecs { Either :: Left (vec) => vec . len () , Either :: Right (list) => list . iter () . map (Vec :: len) . sum () , } }
};
}
