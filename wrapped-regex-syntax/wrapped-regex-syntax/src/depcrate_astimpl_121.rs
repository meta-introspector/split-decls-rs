// Generated macro for impl_121 (impl)
macro_rules! Depcrate_astimpl_121 {
() => {
// Module: crate::ast
// Provides: {"impl_121"}
// Dependencies: {}
impl Group { # [doc = " If this group is non-capturing, then this returns the (possibly empty)"] # [doc = " set of flags. Otherwise, `None` is returned."] pub fn flags (& self) -> Option < & Flags > { match self . kind { GroupKind :: NonCapturing (ref flags) => Some (flags) , _ => None , } } # [doc = " Returns true if and only if this group is capturing."] pub fn is_capturing (& self) -> bool { match self . kind { GroupKind :: CaptureIndex (_) | GroupKind :: CaptureName { .. } => true , GroupKind :: NonCapturing (_) => false , } } # [doc = " Returns the capture index of this group, if this is a capturing group."] # [doc = ""] # [doc = " This returns a capture index precisely when `is_capturing` is `true`."] pub fn capture_index (& self) -> Option < u32 > { match self . kind { GroupKind :: CaptureIndex (i) => Some (i) , GroupKind :: CaptureName { ref name , .. } => Some (name . index) , GroupKind :: NonCapturing (_) => None , } } }
};
}
