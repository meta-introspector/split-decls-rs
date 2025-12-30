// Generated macro for FullDuplex (trait)
macro_rules! Depcrate_spiFullDuplex {
() => {
// Module: crate::spi
// Provides: {"FullDuplex"}
// Dependencies: {}
# [doc = " Full duplex SPI (master mode)."] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " - It's the task of the user of this interface to manage the slave select lines."] # [doc = ""] # [doc = " - Due to how full duplex SPI works each `read` call must be preceded by a `write` call."] # [doc = ""] # [doc = " - `read` calls only return the data received with the last `write` call."] # [doc = "   Previously received data is discarded"] # [doc = ""] # [doc = " - Data is only guaranteed to be clocked out when the `read` call succeeds."] # [doc = "   The slave select line shouldn't be released before that."] # [doc = ""] # [doc = " - Some SPIs can work with 8-bit *and* 16-bit words. You can overload this trait with different"] # [doc = "   `Word` types to allow operation in both modes."] pub trait FullDuplex < Word : Copy = u8 > : ErrorType { # [doc = " Reads the word stored in the shift register"] # [doc = ""] # [doc = " **NOTE** A word must be sent to the slave before attempting to call this"] # [doc = " method."] fn read (& mut self) -> nb :: Result < Word , Self :: Error > ; # [doc = " Writes a word to the slave"] fn write (& mut self , word : Word) -> nb :: Result < () , Self :: Error > ; }
};
}
