// Generated macro for impl_10 (impl)
macro_rules! Depcrate_de_errorimpl_10 {
() => {
// Module: crate::de::error
// Provides: {"impl_10"}
// Dependencies: {}
impl < T > From < ciborium_ll :: Error < T > > for Error < T > { # [inline] fn from (value : ciborium_ll :: Error < T >) -> Self { match value { ciborium_ll :: Error :: Io (x) => Self :: Io (x) , ciborium_ll :: Error :: Syntax (x) => Self :: Syntax (x) , } } }
};
}
