// Generated macro for osstring_len (function)
macro_rules! Depcrate_iter_extendosstring_len {
() => {
// Module: crate::iter::extend
// Provides: {"osstring_len"}
// Dependencies: {}
# [doc = " Computes the total OS-string length of a `fast_collect` result."] fn osstring_len < T : AsRef < OsStr > > (vecs : & Either < Vec < T > , LinkedList < Vec < T > > >) -> usize { let osstrs = match vecs { Either :: Left (vec) => Either :: Left (vec . iter ()) , Either :: Right (list) => Either :: Right (list . iter () . flatten ()) , } ; osstrs . map (AsRef :: as_ref) . map (OsStr :: len) . sum () }
};
}
