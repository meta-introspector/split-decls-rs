// Generated macro for parse_encoded_pointer (function)
macro_rules! Depcrate_read_cfiparse_encoded_pointer {
() => {
// Module: crate::read::cfi
// Provides: {"parse_encoded_pointer"}
// Dependencies: {}
fn parse_encoded_pointer < R : Reader > (encoding : constants :: DwEhPe , parameters : & PointerEncodingParameters < '_ , R > , input : & mut R ,) -> Result < Pointer > { if ! encoding . is_valid_encoding () { return Err (Error :: UnknownPointerEncoding (encoding)) ; } if encoding == constants :: DW_EH_PE_omit { return Err (Error :: CannotParseOmitPointerEncoding) ; } let base = match encoding . application () { constants :: DW_EH_PE_absptr => 0 , constants :: DW_EH_PE_pcrel => { if let Some (section_base) = parameters . bases . section { let offset_from_section = input . offset_from (parameters . section) ; section_base . wrapping_add_sized (offset_from_section . into_u64 () , parameters . address_size) } else { return Err (Error :: PcRelativePointerButSectionBaseIsUndefined) ; } } constants :: DW_EH_PE_textrel => { if let Some (text) = parameters . bases . text { text } else { return Err (Error :: TextRelativePointerButTextBaseIsUndefined) ; } } constants :: DW_EH_PE_datarel => { if let Some (data) = parameters . bases . data { data } else { return Err (Error :: DataRelativePointerButDataBaseIsUndefined) ; } } constants :: DW_EH_PE_funcrel => { if let Some (func) = parameters . func_base { func } else { return Err (Error :: FuncRelativePointerInBadContext) ; } } constants :: DW_EH_PE_aligned => return Err (Error :: UnsupportedPointerEncoding) , _ => unreachable ! () , } ; let offset = parse_encoded_value (encoding , parameters , input) ? ; Ok (Pointer :: new (encoding , base . wrapping_add_sized (offset , parameters . address_size) ,)) }
};
}
