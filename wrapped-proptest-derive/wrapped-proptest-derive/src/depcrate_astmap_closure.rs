// Generated macro for map_closure (function)
macro_rules! Depcrate_astmap_closure {
() => {
// Module: crate::ast
// Provides: {"map_closure"}
// Dependencies: {}
# [doc = " Constructs a `MapClosure` for the given `path` and a list of fields."] pub fn map_closure (path : syn :: Path , fs : & [syn :: Field]) -> MapClosure { MapClosure (path , fs . to_owned ()) }
};
}
