// Generated macro for Table (struct)
macro_rules! Depcrate_hpack_tableTable {
() => {
// Module: crate::hpack::table
// Provides: {"Table"}
// Dependencies: {}
# [doc = " HPACK encoder table"] # [derive (Debug)] pub struct Table { mask : usize , indices : Vec < Option < Pos > > , slots : VecDeque < Slot > , inserted : usize , size : usize , max_size : usize , }
};
}
