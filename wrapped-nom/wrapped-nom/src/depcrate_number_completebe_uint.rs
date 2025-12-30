// Generated macro for be_uint (function)
macro_rules! Depcrate_number_completebe_uint {
() => {
// Module: crate::number::complete
// Provides: {"be_uint"}
// Dependencies: {}
# [inline] fn be_uint < I , Uint , E : ParseError < I > > (input : I , bound : usize) -> IResult < I , Uint , E > where I : Input < Item = u8 > , Uint : Default + Shl < u8 , Output = Uint > + Add < Uint , Output = Uint > + From < u8 > , { super :: be_uint (bound) . parse_complete (input) }
};
}
