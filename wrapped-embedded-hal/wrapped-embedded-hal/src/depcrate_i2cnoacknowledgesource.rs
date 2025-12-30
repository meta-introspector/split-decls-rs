// Generated macro for NoAcknowledgeSource (enum)
macro_rules! Depcrate_i2cNoAcknowledgeSource {
() => {
// Module: crate::i2c
// Provides: {"NoAcknowledgeSource"}
// Dependencies: {}
# [doc = " I2C no acknowledge error source."] # [doc = ""] # [doc = " In cases where it is possible, a device should indicate if a no acknowledge"] # [doc = " response was received to an address versus a no acknowledge to a data byte."] # [doc = " Where it is not possible to differentiate, `Unknown` should be indicated."] # [derive (Debug , Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash)] # [cfg_attr (feature = "defmt-03" , derive (defmt :: Format))] pub enum NoAcknowledgeSource { # [doc = " The device did not acknowledge its address. The device may be missing."] Address , # [doc = " The device did not acknowledge the data. It may not be ready to process"] # [doc = " requests at the moment."] Data , # [doc = " Either the device did not acknowledge its address or the data, but it is"] # [doc = " unknown which."] Unknown , }
};
}
