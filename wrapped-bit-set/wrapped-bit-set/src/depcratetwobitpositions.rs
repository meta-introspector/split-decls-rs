// Generated macro for TwoBitPositions (struct)
macro_rules! DepcrateTwoBitPositions {
() => {
// Module: crate
// Provides: {"TwoBitPositions"}
// Dependencies: {}
# [doc = " An iterator combining two `BitSet` iterators."] # [derive (Clone)] struct TwoBitPositions < 'a , B : 'a > { set : Blocks < 'a , B > , other : Blocks < 'a , B > , merge : fn (B , B) -> B , }
};
}
