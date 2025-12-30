// Generated macro for impl_20 (impl)
macro_rules! Depcrate_stdimpl_20 {
() => {
// Module: crate::std
// Provides: {"impl_20"}
// Dependencies: {}
impl < T : embedded_io :: Write + ? Sized > std :: io :: Write for ToStd < T > { fn write (& mut self , buf : & [u8]) -> Result < usize , std :: io :: Error > { match self . inner . write (buf) { Ok (n) => Ok (n) , Err (e) if e . kind () == embedded_io :: ErrorKind :: WriteZero => Ok (0) , Err (e) => Err (to_std_error (e)) , } } fn write_all (& mut self , buf : & [u8]) -> Result < () , std :: io :: Error > { self . inner . write_all (buf) . map_err (to_std_error) } fn write_fmt (& mut self , fmt : core :: fmt :: Arguments < '_ >) -> Result < () , std :: io :: Error > { match self . inner . write_fmt (fmt) { Ok (()) => Ok (()) , Err (e @ embedded_io :: WriteFmtError :: FmtError) => { Err (std :: io :: Error :: other (format ! ("{e:?}"))) } Err (embedded_io :: WriteFmtError :: Other (e)) => Err (to_std_error (e)) , } } fn flush (& mut self) -> Result < () , std :: io :: Error > { self . inner . flush () . map_err (to_std_error) } }
};
}
