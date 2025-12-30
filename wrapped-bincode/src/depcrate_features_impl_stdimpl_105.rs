// Generated macro for impl_105 (impl)
macro_rules! Depcrate_features_impl_stdimpl_105 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_105"}
// Dependencies: {}
impl < R > Reader for IoReader < R > where R : std :: io :: Read , { # [inline (always)] fn read (& mut self , bytes : & mut [u8]) -> Result < () , DecodeError > { self . reader . read_exact (bytes) . map_err (| inner | DecodeError :: Io { inner , additional : bytes . len () , }) } }
};
}
