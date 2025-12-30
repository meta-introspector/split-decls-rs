// Generated macro for Packet (struct)
macro_rules! DepcratePacket {
() => {
// Module: crate
// Provides: {"Packet"}
// Dependencies: {}
# [cfg_attr (feature = "serde" , cfg_eval :: cfg_eval , serde_as)] # [cfg_attr (feature = "frozen-abi" , derive (AbiExample))] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (Clone , Eq)] # [repr (C)] pub struct Packet { # [cfg_attr (feature = "serde" , serde_as (as = "Bytes"))] buffer : [u8 ; PACKET_DATA_SIZE] , meta : Meta , }
};
}
