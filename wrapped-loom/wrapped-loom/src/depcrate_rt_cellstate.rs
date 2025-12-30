// Generated macro for State (struct)
macro_rules! Depcrate_rt_cellState {
() => {
// Module: crate::rt::cell
// Provides: {"State"}
// Dependencies: {}
# [derive (Debug)] pub (super) struct State { # [doc = " Where the cell was created"] created_location : Location , # [doc = " Number of threads currently reading the cell"] is_reading : usize , # [doc = " `true` if in a `with_mut` closure."] is_writing : bool , # [doc = " The transitive closure of all immutable accesses of `data`."] read_access : VersionVec , # [doc = " Location for the *last* time a thread read from the cell."] read_locations : LocationSet , # [doc = " The last mutable access of `data`."] write_access : VersionVec , # [doc = " Location for the *last* time a thread wrote to the cell"] write_locations : LocationSet , }
};
}
