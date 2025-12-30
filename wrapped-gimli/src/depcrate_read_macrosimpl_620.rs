// Generated macro for impl_620 (impl)
macro_rules! Depcrate_read_macrosimpl_620 {
() => {
// Module: crate::read::macros
// Provides: {"impl_620"}
// Dependencies: {}
impl < R : Reader > MacroUnitHeader < R > { const OFFSET_SIZE_FLAG : u8 = 0b0000_0001 ; const DEBUG_LINE_OFFSET_FLAG : u8 = 0b0000_0010 ; const OPCODE_OPERANDS_TABLE_FLAG : u8 = 0b0000_0100 ; fn parse (input : & mut R) -> Result < Self > { let version = input . read_u16 () ? ; let flags = input . read_u8 () ? ; let format = if flags & Self :: OFFSET_SIZE_FLAG == 0 { Format :: Dwarf32 } else { Format :: Dwarf64 } ; let _debug_line_offset = if flags & Self :: DEBUG_LINE_OFFSET_FLAG != 0 { DebugLineOffset (input . read_offset (format) ?) } else { DebugLineOffset (R :: Offset :: from_u64 (0) ?) } ; if flags & Self :: OPCODE_OPERANDS_TABLE_FLAG != 0 { return Err (Error :: UnsupportedOpcodeOperandsTable) ; } Ok (MacroUnitHeader { _version : version , flags , _debug_line_offset , }) } fn format (& self) -> Format { if self . flags & Self :: OFFSET_SIZE_FLAG == 0 { Format :: Dwarf32 } else { Format :: Dwarf64 } } }
};
}
