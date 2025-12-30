// Generated macro for Write (trait)
macro_rules! Depcrate_serialWrite {
() => {
// Module: crate::serial
// Provides: {"Write"}
// Dependencies: {}
# [doc = " Write half of a serial interface."] pub trait Write < Word : Copy = u8 > : ErrorType { # [doc = " Writes a single word to the serial interface."] fn write (& mut self , word : Word) -> nb :: Result < () , Self :: Error > ; # [doc = " Ensures that none of the previously written words are still buffered."] fn flush (& mut self) -> nb :: Result < () , Self :: Error > ; }
};
}
