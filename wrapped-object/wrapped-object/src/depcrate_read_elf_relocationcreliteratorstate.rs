// Generated macro for CrelIteratorState (struct)
macro_rules! Depcrate_read_elf_relocationCrelIteratorState {
() => {
// Module: crate::read::elf::relocation
// Provides: {"CrelIteratorState"}
// Dependencies: {}
# [derive (Default , Debug , Clone)] struct CrelIteratorState { # [doc = " Index of the current relocation."] index : usize , # [doc = " Offset of the latest relocation."] offset : u64 , # [doc = " Addend of the latest relocation."] addend : i64 , # [doc = " Symbol index of the latest relocation."] symidx : u32 , # [doc = " Type of the latest relocation."] typ : u32 , }
};
}
