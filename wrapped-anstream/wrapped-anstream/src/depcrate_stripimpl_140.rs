// Generated macro for impl_140 (impl)
macro_rules! Depcrate_stripimpl_140 {
() => {
// Module: crate::strip
// Provides: {"impl_140"}
// Dependencies: {}
impl < S > std :: io :: Write for StripStream < S > where S : std :: io :: Write , S : AsLockedWrite , { # [inline] fn write (& mut self , buf : & [u8]) -> std :: io :: Result < usize > { write (& mut self . raw . as_locked_write () , & mut self . state , buf) } # [inline] fn write_vectored (& mut self , bufs : & [std :: io :: IoSlice < '_ >]) -> std :: io :: Result < usize > { let buf = bufs . iter () . find (| b | ! b . is_empty ()) . map (| b | & * * b) . unwrap_or (& [] [..]) ; self . write (buf) } # [inline] fn flush (& mut self) -> std :: io :: Result < () > { self . raw . as_locked_write () . flush () } # [inline] fn write_all (& mut self , buf : & [u8]) -> std :: io :: Result < () > { write_all (& mut self . raw . as_locked_write () , & mut self . state , buf) } # [inline] fn write_fmt (& mut self , args : std :: fmt :: Arguments < '_ >) -> std :: io :: Result < () > { write_fmt (& mut self . raw . as_locked_write () , & mut self . state , args) } }
};
}
