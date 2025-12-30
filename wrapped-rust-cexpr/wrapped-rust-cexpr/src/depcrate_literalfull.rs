// Generated macro for full (function)
macro_rules! Depcrate_literalfull {
() => {
// Module: crate::literal
// Provides: {"full"}
// Dependencies: {}
# [doc = " ensures the child parser consumes the whole input"] pub fn full < I : Clone , O , F > (f : F ,) -> impl Fn (I) -> nom :: IResult < I , O > where I : nom :: InputLength , F : Fn (I) -> nom :: IResult < I , O > , { move | input | { let res = f (input) ; match res { Ok ((i , o)) => { if i . input_len () == 0 { Ok ((i , o)) } else { Err (nom :: Err :: Error (nom :: error :: Error :: new (i , nom :: error :: ErrorKind :: Complete))) } } r => r , } } }
};
}
