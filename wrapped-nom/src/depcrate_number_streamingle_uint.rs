// Generated macro for le_uint (function)
macro_rules! Depcrate_number_streamingle_uint {
() => {
// Module: crate::number::streaming
// Provides: {"le_uint"}
// Dependencies: {}
# [inline] fn le_uint < I , Uint , E : ParseError < I > > (input : I , bound : usize) -> IResult < I , Uint , E > where I : Input < Item = u8 > , Uint : Default + Shl < u8 , Output = Uint > + Add < Uint , Output = Uint > + From < u8 > , { super :: le_uint (bound) . parse (input) }
};
}
