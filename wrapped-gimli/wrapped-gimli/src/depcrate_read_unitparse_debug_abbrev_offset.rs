// Generated macro for parse_debug_abbrev_offset (function)
macro_rules! Depcrate_read_unitparse_debug_abbrev_offset {
() => {
// Module: crate::read::unit
// Provides: {"parse_debug_abbrev_offset"}
// Dependencies: {}
# [doc = " Parse the `debug_abbrev_offset` in the compilation unit header."] fn parse_debug_abbrev_offset < R : Reader > (input : & mut R , format : Format ,) -> Result < DebugAbbrevOffset < R :: Offset > > { input . read_offset (format) . map (DebugAbbrevOffset) }
};
}
