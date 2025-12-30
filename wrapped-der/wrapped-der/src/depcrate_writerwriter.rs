// Generated macro for Writer (trait)
macro_rules! Depcrate_writerWriter {
() => {
// Module: crate::writer
// Provides: {"Writer"}
// Dependencies: {}
# [doc = " Writer trait which outputs encoded DER."] pub trait Writer { # [doc = " Write the given DER-encoded bytes as output."] fn write (& mut self , slice : & [u8]) -> Result < () > ; # [doc = " Write a single byte."] fn write_byte (& mut self , byte : u8) -> Result < () > { self . write (& [byte]) } }
};
}
