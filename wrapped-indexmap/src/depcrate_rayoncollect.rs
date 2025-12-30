// Generated macro for collect (function)
macro_rules! Depcrate_rayoncollect {
() => {
// Module: crate::rayon
// Provides: {"collect"}
// Dependencies: {}
fn collect < I : IntoParallelIterator > (iter : I) -> LinkedList < Vec < I :: Item > > { iter . into_par_iter () . collect_vec_list () }
};
}
