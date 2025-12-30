// Generated macro for LookupTableMeta (struct)
macro_rules! Depcrate_stateLookupTableMeta {
() => {
// Module: crate::state
// Provides: {"LookupTableMeta"}
// Dependencies: {}
# [doc = " Address lookup table metadata"] # [cfg_attr (feature = "frozen-abi" , derive (AbiExample))] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (Debug , PartialEq , Eq , Clone)] pub struct LookupTableMeta { # [doc = " Lookup tables cannot be closed until the deactivation slot is"] # [doc = " no longer \"recent\" (not accessible in the `SlotHashes` sysvar)."] pub deactivation_slot : Slot , # [doc = " The slot that the table was last extended. Address tables may"] # [doc = " only be used to lookup addresses that were extended before"] # [doc = " the current bank's slot."] pub last_extended_slot : Slot , # [doc = " The start index where the table was last extended from during"] # [doc = " the `last_extended_slot`."] pub last_extended_slot_start_index : u8 , # [doc = " Authority address which must sign for each modification."] pub authority : Option < Pubkey > , pub _padding : u16 , }
};
}
