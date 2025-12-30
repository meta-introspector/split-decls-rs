// Generated macro for path_res (function)
macro_rules! Depcratepath_res {
() => {
// Module: crate
// Provides: {"path_res"}
// Dependencies: {}
# [doc = " If `maybe_path` is a path node, resolves it, otherwise returns `Res::Err`"] pub fn path_res < 'tcx > (cx : & LateContext < '_ > , maybe_path : & impl MaybePath < 'tcx >) -> Res { match maybe_path . qpath_opt () { None => Res :: Err , Some (qpath) => cx . qpath_res (qpath , maybe_path . hir_id ()) , } }
};
}
