// Generated macro for parse_type_offset (function)
macro_rules! Depcrate_read_unitparse_type_offset {
() => {
// Module: crate::read::unit
// Provides: {"parse_type_offset"}
// Dependencies: {}
# [doc = " Parse a type unit header's type offset."] fn parse_type_offset < R : Reader > (input : & mut R , format : Format) -> Result < UnitOffset < R :: Offset > > { input . read_offset (format) . map (UnitOffset) }
};
}
