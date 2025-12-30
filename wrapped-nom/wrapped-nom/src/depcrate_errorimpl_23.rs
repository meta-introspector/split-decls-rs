// Generated macro for impl_23 (impl)
macro_rules! Depcrate_errorimpl_23 {
() => {
// Module: crate::error
// Provides: {"impl_23"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < I : ToOwned + ? Sized > Error < & I > { # [doc = " Converts `Error<&I>` into `Error<I::Owned>` by cloning."] pub fn cloned (self) -> Error < I :: Owned > { Error { input : self . input . to_owned () , code : self . code , } } }
};
}
