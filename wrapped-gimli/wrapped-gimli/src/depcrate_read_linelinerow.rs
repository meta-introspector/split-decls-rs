// Generated macro for LineRow (struct)
macro_rules! Depcrate_read_lineLineRow {
() => {
// Module: crate::read::line
// Provides: {"LineRow"}
// Dependencies: {}
# [doc = " A row in the line number program's resulting matrix."] # [doc = ""] # [doc = " Each row is a copy of the registers of the state machine, as defined in section 6.2.2."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub struct LineRow { tombstone : bool , address : u64 , op_index : Wrapping < u64 > , file : u64 , line : Wrapping < u64 > , column : u64 , is_stmt : bool , basic_block : bool , end_sequence : bool , prologue_end : bool , epilogue_begin : bool , isa : u64 , discriminator : u64 , }
};
}
