// Generated macro for parse_debug_info_offset (function)
macro_rules! Depcrate_read_unitparse_debug_info_offset {
() => {
// Module: crate::read::unit
// Provides: {"parse_debug_info_offset"}
// Dependencies: {}
# [doc = " Parse the `debug_info_offset` in the arange header."] pub (crate) fn parse_debug_info_offset < R : Reader > (input : & mut R , format : Format ,) -> Result < DebugInfoOffset < R :: Offset > > { input . read_offset (format) . map (DebugInfoOffset) }
};
}
