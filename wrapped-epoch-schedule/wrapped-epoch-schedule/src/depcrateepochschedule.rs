// Generated macro for EpochSchedule (struct)
macro_rules! DepcrateEpochSchedule {
() => {
// Module: crate
// Provides: {"EpochSchedule"}
// Dependencies: {}
# [repr (C)] # [cfg_attr (feature = "frozen-abi" , derive (solana_frozen_abi_macro :: AbiExample))] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize) , serde (rename_all = "camelCase"))] # [derive (Debug , CloneZeroed , PartialEq , Eq)] pub struct EpochSchedule { # [doc = " The maximum number of slots in each epoch."] pub slots_per_epoch : u64 , # [doc = " A number of slots before beginning of an epoch to calculate"] # [doc = " a leader schedule for that epoch."] pub leader_schedule_slot_offset : u64 , # [doc = " Whether epochs start short and grow."] pub warmup : bool , # [doc = " The first epoch after the warmup period."] # [doc = ""] # [doc = " Basically: `log2(slots_per_epoch) - log2(MINIMUM_SLOTS_PER_EPOCH)`."] pub first_normal_epoch : u64 , # [doc = " The first slot after the warmup period."] # [doc = ""] # [doc = " Basically: `MINIMUM_SLOTS_PER_EPOCH * (2.pow(first_normal_epoch) - 1)`."] pub first_normal_slot : u64 , }
};
}
