// Generated macro for AnyWrite (trait)
macro_rules! Depcrate_writeAnyWrite {
() => {
// Module: crate::write
// Provides: {"AnyWrite"}
// Dependencies: {}
pub trait AnyWrite { type Wstr : ? Sized ; type Error ; fn write_fmt (& mut self , fmt : fmt :: Arguments) -> Result < () , Self :: Error > ; fn write_str (& mut self , s : & Self :: Wstr) -> Result < () , Self :: Error > ; }
};
}
