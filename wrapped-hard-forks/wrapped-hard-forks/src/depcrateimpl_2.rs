// Generated macro for impl_2 (impl)
macro_rules! Depcrateimpl_2 {
() => {
// Module: crate
// Provides: {"impl_2"}
// Dependencies: {}
impl HardForks { pub fn register (& mut self , new_slot : u64) { if let Some (i) = self . hard_forks . iter () . position (| (slot , _) | * slot == new_slot) { self . hard_forks [i] = (new_slot , self . hard_forks [i] . 1 . saturating_add (1)) ; } else { self . hard_forks . push ((new_slot , 1)) ; } # [allow (clippy :: stable_sort_primitive)] self . hard_forks . sort () ; } pub fn iter (& self) -> std :: slice :: Iter < '_ , (u64 , usize) > { self . hard_forks . iter () } pub fn is_empty (& self) -> bool { self . hard_forks . is_empty () } pub fn get_hash_data (& self , slot : u64 , parent_slot : u64) -> Option < [u8 ; 8] > { let fork_count : usize = self . hard_forks . iter () . map (| (fork_slot , fork_count) | { if parent_slot < * fork_slot && slot >= * fork_slot { * fork_count } else { 0 } }) . sum () ; (fork_count > 0) . then (| | (fork_count as u64) . to_le_bytes ()) } }
};
}
