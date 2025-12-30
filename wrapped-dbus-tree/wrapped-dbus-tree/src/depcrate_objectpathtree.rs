// Generated macro for Tree (struct)
macro_rules! Depcrate_objectpathTree {
() => {
// Module: crate::objectpath
// Provides: {"Tree"}
// Dependencies: {}
# [doc = " A collection of object paths."] # [derive (Debug , Default)] pub struct Tree < M : MethodType < D > , D : DataType > { paths : ArcMap < Arc < Path < 'static > > , ObjectPath < M , D > > , data : D :: Tree , }
};
}
