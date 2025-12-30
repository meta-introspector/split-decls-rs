// Generated macro for parse_directory_v5 (function)
macro_rules! Depcrate_read_lineparse_directory_v5 {
() => {
// Module: crate::read::line
// Provides: {"parse_directory_v5"}
// Dependencies: {}
fn parse_directory_v5 < R : Reader > (input : & mut R , encoding : Encoding , formats : & [FileEntryFormat] ,) -> Result < AttributeValue < R > > { let mut path_name = None ; for format in formats { let value = parse_attribute (input , encoding , format . form) ? ; if format . content_type == constants :: DW_LNCT_path { path_name = Some (value) ; } } Ok (path_name . unwrap ()) }
};
}
