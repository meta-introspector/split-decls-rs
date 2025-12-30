// Generated macro for BandRef (enum)
macro_rules! DepcrateBandRef {
() => {
// Module: crate
// Provides: {"BandRef"}
// Dependencies: {}
# [doc = " A band in a sideband channel."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum BandRef < 'a > { # [doc = " A band carrying data."] Data (& 'a [u8]) , # [doc = " A band carrying user readable progress information."] Progress (& 'a [u8]) , # [doc = " A band carrying user readable errors."] Error (& 'a [u8]) , }
};
}
