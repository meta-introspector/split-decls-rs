// Generated macro for UninitVector (type)
macro_rules! Depcrate_base_aliasUninitVector {
() => {
// Module: crate::base::alias
// Provides: {"UninitVector"}
// Dependencies: {}
# [doc = " An owned matrix with uninitialized data."] pub type UninitVector < T , D > = Matrix < MaybeUninit < T > , D , U1 , OwnedUninit < T , D , U1 > > ;
};
}
