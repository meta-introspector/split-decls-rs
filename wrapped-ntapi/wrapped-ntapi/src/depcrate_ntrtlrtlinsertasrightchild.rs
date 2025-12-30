// Generated macro for RtlInsertAsRightChild (function)
macro_rules! Depcrate_ntrtlRtlInsertAsRightChild {
() => {
// Module: crate::ntrtl
// Provides: {"RtlInsertAsRightChild"}
// Dependencies: {}
# [inline] pub fn RtlInsertAsRightChild (ParentLinks : & mut RTL_SPLAY_LINKS , ChildLinks : & mut RTL_SPLAY_LINKS ,) { ParentLinks . RightChild = ChildLinks ; ChildLinks . Parent = ParentLinks ; }
};
}
