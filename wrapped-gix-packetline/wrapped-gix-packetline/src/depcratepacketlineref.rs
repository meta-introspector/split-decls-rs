// Generated macro for PacketLineRef (enum)
macro_rules! DepcratePacketLineRef {
() => {
// Module: crate
// Provides: {"PacketLineRef"}
// Dependencies: {}
# [doc = " A borrowed packet line as it refers to a slice of data by reference."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum PacketLineRef < 'a > { # [doc = " A chunk of raw data."] Data (& 'a [u8]) , # [doc = " A flush packet."] Flush , # [doc = " A delimiter packet."] Delimiter , # [doc = " The end of the response."] ResponseEnd , }
};
}
