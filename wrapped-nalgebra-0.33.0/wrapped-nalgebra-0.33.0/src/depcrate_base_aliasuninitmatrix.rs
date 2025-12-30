// Generated macro for UninitMatrix (type)
macro_rules! Depcrate_base_aliasUninitMatrix {
() => {
// Module: crate::base::alias
// Provides: {"UninitMatrix"}
// Dependencies: {}
# [doc = " An owned matrix with uninitialized data."] pub type UninitMatrix < T , R , C > = Matrix < MaybeUninit < T > , R , C , OwnedUninit < T , R , C > > ;
};
}
