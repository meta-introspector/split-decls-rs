// Generated macro for RtlInsertAsLeftChild (function)
macro_rules! Depcrate_ntrtlRtlInsertAsLeftChild {
() => {
// Module: crate::ntrtl
// Provides: {"RtlInsertAsLeftChild"}
// Dependencies: {}
# [inline] pub fn RtlInsertAsLeftChild (ParentLinks : & mut RTL_SPLAY_LINKS , ChildLinks : & mut RTL_SPLAY_LINKS ,) { ParentLinks . LeftChild = ChildLinks ; ChildLinks . Parent = ParentLinks ; }
};
}
