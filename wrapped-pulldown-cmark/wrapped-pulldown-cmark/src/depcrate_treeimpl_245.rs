// Generated macro for impl_245 (impl)
macro_rules! Depcrate_treeimpl_245 {
() => {
// Module: crate::tree
// Provides: {"impl_245"}
// Dependencies: {}
impl Add < usize > for TreeIndex { type Output = TreeIndex ; fn add (self , rhs : usize) -> Self { let inner = self . 0 . get () + rhs ; TreeIndex :: new (inner) } }
};
}
