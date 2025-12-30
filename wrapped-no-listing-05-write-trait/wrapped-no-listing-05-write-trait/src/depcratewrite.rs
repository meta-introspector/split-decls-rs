// Generated macro for Write (trait)
macro_rules! DepcrateWrite {
() => {
// Module: crate
// Provides: {"Write"}
// Dependencies: {}
pub trait Write { fn write (& mut self , buf : & [u8]) -> Result < usize , Error > ; fn flush (& mut self) -> Result < () , Error > ; fn write_all (& mut self , buf : & [u8]) -> Result < () , Error > ; fn write_fmt (& mut self , fmt : fmt :: Arguments) -> Result < () , Error > ; }
};
}
