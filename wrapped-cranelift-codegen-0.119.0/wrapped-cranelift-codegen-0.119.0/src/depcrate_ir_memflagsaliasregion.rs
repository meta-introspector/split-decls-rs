// Generated macro for AliasRegion (enum)
macro_rules! Depcrate_ir_memflagsAliasRegion {
() => {
// Module: crate::ir::memflags
// Provides: {"AliasRegion"}
// Dependencies: {}
# [doc = " Which disjoint region of aliasing memory is accessed in this memory"] # [doc = " operation."] # [derive (Clone , Copy , PartialEq , Eq , Debug , Hash)] # [repr (u8)] # [allow (missing_docs)] # [rustfmt :: skip] pub enum AliasRegion { Heap = 0b01 , Table = 0b10 , Vmctx = 0b11 , }
};
}
