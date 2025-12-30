// Generated macro for EpochInfo (struct)
macro_rules! DepcrateEpochInfo {
() => {
// Module: crate
// Provides: {"EpochInfo"}
// Dependencies: {}
# [cfg_attr (feature = "serde" , derive (serde_derive :: Deserialize , serde_derive :: Serialize) , serde (rename_all = "camelCase"))] # [derive (Clone , Debug , Eq , PartialEq)] pub struct EpochInfo { # [doc = " The current epoch"] pub epoch : u64 , # [doc = " The current slot, relative to the start of the current epoch"] pub slot_index : u64 , # [doc = " The number of slots in this epoch"] pub slots_in_epoch : u64 , # [doc = " The absolute current slot"] pub absolute_slot : u64 , # [doc = " The current block height"] pub block_height : u64 , # [doc = " Total number of transactions processed without error since genesis"] pub transaction_count : Option < u64 > , }
};
}
