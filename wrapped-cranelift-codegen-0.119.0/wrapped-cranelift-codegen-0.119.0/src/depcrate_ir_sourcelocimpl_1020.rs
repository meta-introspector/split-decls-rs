// Generated macro for impl_1020 (impl)
macro_rules! Depcrate_ir_sourcelocimpl_1020 {
() => {
// Module: crate::ir::sourceloc
// Provides: {"impl_1020"}
// Dependencies: {}
impl SourceLoc { # [doc = " Create a new source location with the given bits."] pub fn new (bits : u32) -> Self { Self (bits) } # [doc = " Is this the default source location?"] pub fn is_default (self) -> bool { self == Default :: default () } # [doc = " Read the bits of this source location."] pub fn bits (self) -> u32 { self . 0 } }
};
}
