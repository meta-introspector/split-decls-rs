// Generated macro for macro_9 (macro)
macro_rules! Depcratemacro_9 {
() => {
// Module: crate
// Provides: {"macro_9"}
// Dependencies: {}
bitflags ! { # [repr (C)] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub struct PacketFlags : u8 { const DISCARD = 0b0000_0001 ; const FORWARDED = 0b0000_0010 ; const REPAIR = 0b0000_0100 ; const SIMPLE_VOTE_TX = 0b0000_1000 ; const UNUSED_0 = 0b0001_0000 ; const UNUSED_1 = 0b0010_0000 ; # [doc = " For tracking performance"] const PERF_TRACK_PACKET = 0b0100_0000 ; # [doc = " For marking packets from staked nodes"] const FROM_STAKED_NODE = 0b1000_0000 ; } }
};
}
