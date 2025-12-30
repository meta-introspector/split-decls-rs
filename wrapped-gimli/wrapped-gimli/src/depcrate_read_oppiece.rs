// Generated macro for Piece (struct)
macro_rules! Depcrate_read_opPiece {
() => {
// Module: crate::read::op
// Provides: {"Piece"}
// Dependencies: {}
# [doc = " The description of a single piece of the result of a DWARF"] # [doc = " expression."] # [derive (Debug , Clone , Copy , PartialEq)] pub struct Piece < R , Offset = < R as Reader > :: Offset > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { # [doc = " If given, the size of the piece in bits.  If `None`, there"] # [doc = " must be only one piece whose size is all of the object."] pub size_in_bits : Option < u64 > , # [doc = " If given, the bit offset of the piece within the location."] # [doc = " If the location is a `Location::Register` or `Location::Value`,"] # [doc = " then this offset is from the least significant bit end of"] # [doc = " the register or value."] # [doc = " If the location is a `Location::Address` then the offset uses"] # [doc = " the bit numbering and direction conventions of the language"] # [doc = " and target system."] # [doc = ""] # [doc = " If `None`, the piece starts at the location. If the"] # [doc = " location is a register whose size is larger than the piece,"] # [doc = " then placement within the register is defined by the ABI."] pub bit_offset : Option < u64 > , # [doc = " Where this piece is to be found."] pub location : Location < R , Offset > , }
};
}
