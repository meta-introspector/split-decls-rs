// Generated macro for impl_14 (impl)
macro_rules! Depcrate_stdimpl_14 {
() => {
// Module: crate::std
// Provides: {"impl_14"}
// Dependencies: {}
# [deny (clippy :: missing_trait_methods , reason = "Methods should be forwarded to the underlying type")] impl < T : std :: io :: Write + ? Sized > embedded_io :: Write for FromStd < T > { fn write (& mut self , buf : & [u8]) -> Result < usize , Self :: Error > { match self . inner . write (buf) { Ok (0) if ! buf . is_empty () => Err (std :: io :: ErrorKind :: WriteZero . into ()) , Ok (n) => Ok (n) , Err (e) => Err (e) , } } fn write_all (& mut self , buf : & [u8]) -> Result < () , Self :: Error > { self . inner . write_all (buf) } fn write_fmt (& mut self , fmt : core :: fmt :: Arguments < '_ > ,) -> Result < () , embedded_io :: WriteFmtError < Self :: Error > > { Ok (self . inner . write_fmt (fmt) ?) } fn flush (& mut self) -> Result < () , Self :: Error > { self . inner . flush () } }
};
}
