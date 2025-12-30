// Generated macro for impl_2006 (impl)
macro_rules! Depcrate_serializeimpl_2006 {
() => {
// Module: crate::serialize
// Provides: {"impl_2006"}
// Dependencies: {}
impl < DB > Write for Output < '_ , '_ , DB > where for < 'c > DB : Backend < BindCollector < 'c > = RawBytesBindCollector < DB > > , { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . out . 0 . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . out . 0 . flush () } fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { self . out . 0 . write_all (buf) } fn write_fmt (& mut self , fmt : fmt :: Arguments < '_ >) -> io :: Result < () > { self . out . 0 . write_fmt (fmt) } }
};
}
