// Generated macro for Meta (struct)
macro_rules! DepcrateMeta {
() => {
// Module: crate
// Provides: {"Meta"}
// Dependencies: {}
# [cfg_attr (feature = "frozen-abi" , derive (AbiExample))] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (Clone , Debug , PartialEq , Eq)] # [repr (C)] pub struct Meta { pub size : usize , pub addr : IpAddr , pub port : u16 , pub flags : PacketFlags , remote_pubkey : Pubkey , }
};
}
