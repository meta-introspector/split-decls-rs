// Generated macro for Location (enum)
macro_rules! Depcrate_read_opLocation {
() => {
// Module: crate::read::op
// Provides: {"Location"}
// Dependencies: {}
# [doc = " A single location of a piece of the result of a DWARF expression."] # [derive (Debug , Clone , Copy , PartialEq)] pub enum Location < R , Offset = < R as Reader > :: Offset > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { # [doc = " The piece is empty.  Ordinarily this means the piece has been"] # [doc = " optimized away."] Empty , # [doc = " The piece is found in a register."] Register { # [doc = " The register number."] register : Register , } , # [doc = " The piece is found in memory."] Address { # [doc = " The address."] address : u64 , } , # [doc = " The piece has no location but its value is known."] Value { # [doc = " The value."] value : Value , } , # [doc = " The piece is represented by some constant bytes."] Bytes { # [doc = " The value."] value : R , } , # [doc = " The piece is a pointer to a value which has no actual location."] ImplicitPointer { # [doc = " The `.debug_info` offset of the value that this is an implicit pointer into."] value : DebugInfoOffset < Offset > , # [doc = " The byte offset into the value that the implicit pointer points to."] byte_offset : i64 , } , }
};
}
