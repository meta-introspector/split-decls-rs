// Generated macro for new_tree (function)
macro_rules! Depcrate_objectpathnew_tree {
() => {
// Module: crate::objectpath
// Provides: {"new_tree"}
// Dependencies: {}
pub fn new_tree < M : MethodType < D > , D : DataType > (d : D :: Tree) -> Tree < M , D > { Tree { paths : ArcMap :: new () , data : d } }
};
}
