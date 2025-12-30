// Generated macro for impl_246 (impl)
macro_rules! Depcrate_treeimpl_246 {
() => {
// Module: crate::tree
// Provides: {"impl_246"}
// Dependencies: {}
impl Sub < usize > for TreeIndex { type Output = TreeIndex ; fn sub (self , rhs : usize) -> Self { let inner = self . 0 . get () . checked_sub (rhs) . unwrap () ; TreeIndex :: new (inner) } }
};
}
