// Generated macro for Encoding (enum)
macro_rules! DepcrateEncoding {
() => {
// Module: crate
// Provides: {"Encoding"}
// Dependencies: {}
# [doc = " How a defmt frame is encoded"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [non_exhaustive] pub enum Encoding { # [doc = " raw data, that is no encoding."] Raw , # [doc = " [Reverse Zero-compressing COBS encoding](https://github.com/Dirbaio/rzcobs)"] Rzcobs , }
};
}
