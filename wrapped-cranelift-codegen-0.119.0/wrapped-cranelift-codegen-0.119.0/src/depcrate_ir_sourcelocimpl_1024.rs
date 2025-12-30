// Generated macro for impl_1024 (impl)
macro_rules! Depcrate_ir_sourcelocimpl_1024 {
() => {
// Module: crate::ir::sourceloc
// Provides: {"impl_1024"}
// Dependencies: {}
impl RelSourceLoc { # [doc = " Create a new relative source location with the given bits."] pub fn new (bits : u32) -> Self { Self (bits) } # [doc = " Creates a new `RelSourceLoc` based on the given base and offset."] pub fn from_base_offset (base : SourceLoc , offset : SourceLoc) -> Self { if base . is_default () || offset . is_default () { Self :: default () } else { Self (offset . bits () . wrapping_sub (base . bits ())) } } # [doc = " Expands the relative source location into an absolute one, using the given base."] pub fn expand (& self , base : SourceLoc) -> SourceLoc { if self . is_default () || base . is_default () { Default :: default () } else { SourceLoc :: new (self . 0 . wrapping_add (base . bits ())) } } # [doc = " Is this the default relative source location?"] pub fn is_default (self) -> bool { self == Default :: default () } }
};
}
