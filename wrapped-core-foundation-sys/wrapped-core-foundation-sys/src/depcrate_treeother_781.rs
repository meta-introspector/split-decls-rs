// Generated macro for other_781 (other)
macro_rules! Depcrate_treeother_781 {
() => {
// Module: crate::tree
// Provides: {"other_781"}
// Dependencies: {}
unsafe extern "C" { pub fn CFTreeCreate (allocator : CFAllocatorRef , context : * const CFTreeContext) -> CFTreeRef ; pub fn CFTreeAppendChild (tree : CFTreeRef , newChild : CFTreeRef) ; pub fn CFTreeInsertSibling (tree : CFTreeRef , newSibling : CFTreeRef) ; pub fn CFTreeRemoveAllChildren (tree : CFTreeRef) ; pub fn CFTreePrependChild (tree : CFTreeRef , newChild : CFTreeRef) ; pub fn CFTreeRemove (tree : CFTreeRef) ; pub fn CFTreeSetContext (tree : CFTreeRef , context : * const CFTreeContext) ; pub fn CFTreeSortChildren (tree : CFTreeRef , comparator : CFComparatorFunction , context : * mut c_void ,) ; pub fn CFTreeFindRoot (tree : CFTreeRef) -> CFTreeRef ; pub fn CFTreeGetChildAtIndex (tree : CFTreeRef , idx : CFIndex) -> CFTreeRef ; pub fn CFTreeGetChildCount (tree : CFTreeRef) -> CFIndex ; pub fn CFTreeGetChildren (tree : CFTreeRef , children : * mut CFTreeRef) ; pub fn CFTreeGetContext (tree : CFTreeRef , context : * mut CFTreeContext) ; pub fn CFTreeGetFirstChild (tree : CFTreeRef) -> CFTreeRef ; pub fn CFTreeGetNextSibling (tree : CFTreeRef) -> CFTreeRef ; pub fn CFTreeGetParent (tree : CFTreeRef) -> CFTreeRef ; pub fn CFTreeApplyFunctionToChildren (tree : CFTreeRef , applier : CFTreeApplierFunction , context : * mut c_void ,) ; pub fn CFTreeGetTypeID () -> CFTypeID ; }
};
}
