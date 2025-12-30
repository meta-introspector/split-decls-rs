// Generated macro for impl_29 (impl)
macro_rules! Depcrate_stateimpl_29 {
() => {
// Module: crate::state
// Provides: {"impl_29"}
// Dependencies: {}
impl LookupTableMeta { pub fn new (authority : Pubkey) -> Self { LookupTableMeta { authority : Some (authority) , .. LookupTableMeta :: default () } } # [doc = " Returns whether the table is considered active for address lookups"] pub fn is_active (& self , current_slot : Slot , slot_hashes : & SlotHashes) -> bool { match self . status (current_slot , slot_hashes) { LookupTableStatus :: Activated => true , LookupTableStatus :: Deactivating { .. } => true , LookupTableStatus :: Deactivated => false , } } # [doc = " Return the current status of the lookup table"] pub fn status (& self , current_slot : Slot , slot_hashes : & SlotHashes) -> LookupTableStatus { if self . deactivation_slot == Slot :: MAX { LookupTableStatus :: Activated } else if self . deactivation_slot == current_slot { LookupTableStatus :: Deactivating { remaining_blocks : MAX_ENTRIES . saturating_add (1) , } } else if let Some (slot_hash_position) = slot_hashes . position (& self . deactivation_slot) { LookupTableStatus :: Deactivating { remaining_blocks : MAX_ENTRIES . saturating_sub (slot_hash_position) , } } else { LookupTableStatus :: Deactivated } } }
};
}
