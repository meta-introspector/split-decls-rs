// Generated macro for tag (function)
macro_rules! Depcrate_bits_streamingtag {
() => {
// Module: crate::bits::streaming
// Provides: {"tag"}
// Dependencies: {}
# [doc = " Generates a parser taking `count` bits and comparing them to `pattern`"] pub fn tag < I , O , C , E : ParseError < (I , usize) > > (pattern : O , count : C ,) -> impl Fn ((I , usize)) -> IResult < (I , usize) , O , E > where I : Input < Item = u8 > + Clone , C : ToUsize , O : From < u8 > + AddAssign + Shl < usize , Output = O > + Shr < usize , Output = O > + PartialEq , { let count = count . to_usize () ; move | input : (I , usize) | { let inp = input . clone () ; take (count) (input) . and_then (| (i , o) | { if pattern == o { Ok ((i , o)) } else { Err (Err :: Error (error_position ! (inp , ErrorKind :: TagBits))) } }) } }
};
}
