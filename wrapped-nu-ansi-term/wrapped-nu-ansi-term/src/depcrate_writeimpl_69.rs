// Generated macro for impl_69 (impl)
macro_rules! Depcrate_writeimpl_69 {
() => {
// Module: crate::write
// Provides: {"impl_69"}
// Dependencies: {}
impl < 'a > AnyWrite for dyn fmt :: Write + 'a { type Wstr = str ; type Error = fmt :: Error ; fn write_fmt (& mut self , fmt : fmt :: Arguments) -> Result < () , Self :: Error > { fmt :: Write :: write_fmt (self , fmt) } fn write_str (& mut self , s : & Self :: Wstr) -> Result < () , Self :: Error > { fmt :: Write :: write_str (self , s) } }
};
}
