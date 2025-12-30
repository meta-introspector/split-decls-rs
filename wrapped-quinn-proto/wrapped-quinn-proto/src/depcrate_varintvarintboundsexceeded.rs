// Generated macro for VarIntBoundsExceeded (struct)
macro_rules! Depcrate_varintVarIntBoundsExceeded {
() => {
// Module: crate::varint
// Provides: {"VarIntBoundsExceeded"}
// Dependencies: {}
# [doc = " Error returned when constructing a `VarInt` from a value >= 2^62"] # [derive (Debug , Copy , Clone , Eq , PartialEq , Error)] # [error ("value too large for varint encoding")] pub struct VarIntBoundsExceeded ;
};
}
