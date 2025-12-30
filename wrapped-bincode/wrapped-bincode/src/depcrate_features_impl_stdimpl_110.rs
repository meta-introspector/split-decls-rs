// Generated macro for impl_110 (impl)
macro_rules! Depcrate_features_impl_stdimpl_110 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_110"}
// Dependencies: {}
impl < W : std :: io :: Write > Writer for IoWriter < '_ , W > { # [inline (always)] fn write (& mut self , bytes : & [u8]) -> Result < () , EncodeError > { self . writer . write_all (bytes) . map_err (| inner | EncodeError :: Io { inner , index : self . bytes_written , }) ? ; self . bytes_written += bytes . len () ; Ok (()) } }
};
}
