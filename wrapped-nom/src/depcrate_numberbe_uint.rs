// Generated macro for be_uint (function)
macro_rules! Depcrate_numberbe_uint {
() => {
// Module: crate::number
// Provides: {"be_uint"}
// Dependencies: {}
# [doc = " creates a big endian unsigned integer parser"] # [doc = ""] # [doc = " * `bound`: the number of bytes that will be read"] # [doc = " * `Uint`: the output type"] # [inline] fn be_uint < I , Uint , E : ParseError < I > > (bound : usize) -> impl Parser < I , Output = Uint , Error = E > where I : Input < Item = u8 > , Uint : Default + Shl < u8 , Output = Uint > + Add < Uint , Output = Uint > + From < u8 > , { BeUint { bound , e : PhantomData , u : PhantomData , } }
};
}
