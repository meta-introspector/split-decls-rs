// Generated macro for le_uint (function)
macro_rules! Depcrate_numberle_uint {
() => {
// Module: crate::number
// Provides: {"le_uint"}
// Dependencies: {}
# [doc = " creates a little endian unsigned integer parser"] # [doc = ""] # [doc = " * `bound`: the number of bytes that will be read"] # [doc = " * `Uint`: the output type"] # [inline] fn le_uint < I , Uint , E : ParseError < I > > (bound : usize) -> impl Parser < I , Output = Uint , Error = E > where I : Input < Item = u8 > , Uint : Default + Shl < u8 , Output = Uint > + Add < Uint , Output = Uint > + From < u8 > , { LeUint { bound , e : PhantomData , u : PhantomData , } }
};
}
