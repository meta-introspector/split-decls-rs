// Generated macro for Buffer (struct)
macro_rules! Depcrate_connection_assemblerBuffer {
() => {
// Module: crate::connection::assembler
// Provides: {"Buffer"}
// Dependencies: {}
# [derive (Debug , Eq)] struct Buffer { offset : u64 , bytes : Bytes , # [doc = " Size of the allocation behind `bytes`, if `defragmented == false`."] # [doc = " Otherwise this will be set to `bytes.len()` by `try_mark_defragment`."] # [doc = " Will never be less than `bytes.len()`."] allocation_size : usize , defragmented : bool , }
};
}
