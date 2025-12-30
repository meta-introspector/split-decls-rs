// Generated macro for parse_encoded_value (function)
macro_rules! Depcrate_read_cfiparse_encoded_value {
() => {
// Module: crate::read::cfi
// Provides: {"parse_encoded_value"}
// Dependencies: {}
fn parse_encoded_value < R : Reader > (encoding : constants :: DwEhPe , parameters : & PointerEncodingParameters < '_ , R > , input : & mut R ,) -> Result < u64 > { match encoding . format () { constants :: DW_EH_PE_absptr => input . read_address (parameters . address_size) , constants :: DW_EH_PE_uleb128 => input . read_uleb128 () , constants :: DW_EH_PE_udata2 => input . read_u16 () . map (u64 :: from) , constants :: DW_EH_PE_udata4 => input . read_u32 () . map (u64 :: from) , constants :: DW_EH_PE_udata8 => input . read_u64 () , constants :: DW_EH_PE_sleb128 => input . read_sleb128 () . map (| a | a as u64) , constants :: DW_EH_PE_sdata2 => input . read_i16 () . map (| a | a as u64) , constants :: DW_EH_PE_sdata4 => input . read_i32 () . map (| a | a as u64) , constants :: DW_EH_PE_sdata8 => input . read_i64 () . map (| a | a as u64) , _ => unreachable ! () , } }
};
}
