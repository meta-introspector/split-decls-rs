// Generated macro for impl_24 (impl)
macro_rules! Depcrate_errorimpl_24 {
() => {
// Module: crate::error
// Provides: {"impl_24"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < I : ToOwned + ? Sized > Error < & mut I > { # [doc = " Converts `Error<&mut I>` into `Error<I::Owned>` by cloning."] pub fn cloned (self) -> Error < I :: Owned > { Error { input : self . input . to_owned () , code : self . code , } } }
};
}
