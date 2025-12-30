// Generated macro for Operation (enum)
macro_rules! Depcrate_i2cOperation {
() => {
// Module: crate::i2c
// Provides: {"Operation"}
// Dependencies: {}
# [doc = " I2C operation."] # [doc = ""] # [doc = " Several operations can be combined as part of a transaction."] # [derive (Debug , PartialEq , Eq)] # [cfg_attr (feature = "defmt-03" , derive (defmt :: Format))] pub enum Operation < 'a > { # [doc = " Read data into the provided buffer."] Read (& 'a mut [u8]) , # [doc = " Write data from the provided buffer."] Write (& 'a [u8]) , }
};
}
