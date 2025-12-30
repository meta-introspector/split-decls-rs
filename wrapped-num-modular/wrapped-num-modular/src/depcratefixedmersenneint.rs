// Generated macro for FixedMersenneInt (type)
macro_rules! DepcrateFixedMersenneInt {
() => {
// Module: crate
// Provides: {"FixedMersenneInt"}
// Dependencies: {}
# [doc = " An integer in modulo ring with a fixed (pseudo) Mersenne number as modulus"] pub type FixedMersenneInt < const P : u8 , const K : umax > = ReducedInt < umax , FixedMersenne < P , K > > ;
};
}
