// Generated macro for impl_90 (impl)
macro_rules! Depcrate_once_cellimpl_90 {
() => {
// Module: crate::once_cell
// Provides: {"impl_90"}
// Dependencies: {}
impl From < usize > for State { fn from (val : usize) -> Self { match val { 0 => State :: Uninitialized , 1 => State :: Initializing , 2 => State :: Initialized , _ => unreachable ! ("Invalid state") , } } }
};
}
