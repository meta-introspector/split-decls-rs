// Generated macro for SmallVec (struct)
macro_rules! DepcrateSmallVec {
() => {
// Module: crate
// Provides: {"SmallVec"}
// Dependencies: {}
# [repr (C)] pub struct SmallVec < T , const N : usize > { len : TaggedLen , raw : RawSmallVec < T , N > , _marker : PhantomData < T > , }
};
}
