// Generated macro for SubBlockSize (type)
macro_rules! DepcrateSubBlockSize {
() => {
// Module: crate
// Provides: {"SubBlockSize"}
// Dependencies: {}
# [doc = " Alias for `SubBlockSize<A, B> = Diff<T, B::BlockSize>`"] pub type SubBlockSize < T , B > = Diff < T , < B as BlockSizeUser > :: BlockSize > ;
};
}
