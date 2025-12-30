// Generated macro for macro_229 (macro)
macro_rules! Depcrate_sourcemacro_229 {
() => {
// Module: crate::source
// Provides: {"macro_229"}
// Dependencies: {}
enum_with_val ! { # [doc = " Events involving a change to a file system object."] # [derive (PartialEq , Eq , Clone , Copy)] pub struct dispatch_source_vnode_flags_t (pub c_ulong) { DISPATCH_VNODE_DELETE = 0x1 , DISPATCH_VNODE_WRITE = 0x2 , DISPATCH_VNODE_EXTEND = 0x4 , DISPATCH_VNODE_ATTRIB = 0x8 , DISPATCH_VNODE_LINK = 0x10 , DISPATCH_VNODE_RENAME = 0x20 , DISPATCH_VNODE_REVOKE = 0x40 , DISPATCH_VNODE_FUNLOCK = 0x100 , } }
};
}
