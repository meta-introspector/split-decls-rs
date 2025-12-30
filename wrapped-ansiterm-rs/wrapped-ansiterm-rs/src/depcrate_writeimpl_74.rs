// Generated macro for impl_74 (impl)
macro_rules! Depcrate_writeimpl_74 {
() => {
// Module: crate::write
// Provides: {"impl_74"}
// Dependencies: {}
impl < 'a > AnyWrite for dyn io :: Write + 'a { type Wstr = [u8] ; type Error = io :: Error ; fn write_fmt (& mut self , fmt : fmt :: Arguments) -> Result < () , Self :: Error > { io :: Write :: write_fmt (self , fmt) } fn write_str (& mut self , s : & Self :: Wstr) -> Result < () , Self :: Error > { io :: Write :: write_all (self , s) } }
};
}
