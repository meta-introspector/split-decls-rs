// Generated macro for Table (trait)
macro_rules! Depcrate_constant_hashTable {
() => {
// Module: crate::constant_hash
// Provides: {"Table"}
// Dependencies: {}
# [doc = " Trait that must be implemented by the entries in a constant hash table."] pub trait Table < K : Copy + Eq > { # [doc = " Get the number of entries in this table which must be a power of two."] fn len (& self) -> usize ; # [doc = " Get the key corresponding to the entry at `idx`, or `None` if the entry is empty."] # [doc = " The `idx` must be in range."] fn key (& self , idx : usize) -> Option < K > ; }
};
}
