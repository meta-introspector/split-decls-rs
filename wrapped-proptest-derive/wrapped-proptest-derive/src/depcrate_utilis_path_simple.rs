// Generated macro for is_path_simple (function)
macro_rules! Depcrate_utilis_path_simple {
() => {
// Module: crate::util
// Provides: {"is_path_simple"}
// Dependencies: {}
# [doc = " Returns true iff the path is simple, i.e:"] # [doc = " just a :: separated list of identifiers."] fn is_path_simple (path : & syn :: Path) -> bool { path . segments . iter () . all (| ps | ps . arguments . is_empty ()) }
};
}
