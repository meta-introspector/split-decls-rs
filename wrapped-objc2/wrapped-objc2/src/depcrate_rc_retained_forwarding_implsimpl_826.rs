// Generated macro for impl_826 (impl)
macro_rules! Depcrate_rc_retained_forwarding_implsimpl_826 {
() => {
// Module: crate::rc::retained_forwarding_impls
// Provides: {"impl_826"}
// Dependencies: {}
# [cfg (feature = "std")] impl < 'a , T : ? Sized > io :: Write for & 'a Retained < T > where & 'a T : io :: Write , { # [inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { (& * * * self) . write (buf) } # [inline] fn write_vectored (& mut self , bufs : & [io :: IoSlice < '_ >]) -> io :: Result < usize > { (& * * * self) . write_vectored (bufs) } # [inline] fn flush (& mut self) -> io :: Result < () > { (& * * * self) . flush () } # [inline] fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { (& * * * self) . write_all (buf) } # [inline] fn write_fmt (& mut self , fmt : fmt :: Arguments < '_ >) -> io :: Result < () > { (& * * * self) . write_fmt (fmt) } }
};
}
