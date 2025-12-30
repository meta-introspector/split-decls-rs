// Generated macro for Operation (enum)
macro_rules! Depcrate_spiOperation {
() => {
// Module: crate::spi
// Provides: {"Operation"}
// Dependencies: {}
# [doc = " SPI transaction operation."] # [doc = ""] # [doc = " This allows composition of SPI operations into a single bus transaction."] # [derive (Debug , PartialEq , Eq)] # [cfg_attr (feature = "defmt-03" , derive (defmt :: Format))] pub enum Operation < 'a , Word : 'static > { # [doc = " Read data into the provided buffer."] # [doc = ""] # [doc = " Equivalent to [`SpiBus::read`]."] Read (& 'a mut [Word]) , # [doc = " Write data from the provided buffer, discarding read data."] # [doc = ""] # [doc = " Equivalent to [`SpiBus::write`]."] Write (& 'a [Word]) , # [doc = " Read data into the first buffer, while writing data from the second buffer."] # [doc = ""] # [doc = " Equivalent to [`SpiBus::transfer`]."] Transfer (& 'a mut [Word] , & 'a [Word]) , # [doc = " Write data out while reading data into the provided buffer."] # [doc = ""] # [doc = " Equivalent to [`SpiBus::transfer_in_place`]."] TransferInPlace (& 'a mut [Word]) , # [doc = " Delay for at least the specified number of nanoseconds."] DelayNs (u32) , }
};
}
