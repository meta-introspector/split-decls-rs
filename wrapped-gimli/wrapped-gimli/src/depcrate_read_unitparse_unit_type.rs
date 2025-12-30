// Generated macro for parse_unit_type (function)
macro_rules! Depcrate_read_unitparse_unit_type {
() => {
// Module: crate::read::unit
// Provides: {"parse_unit_type"}
// Dependencies: {}
# [doc = " Parse the unit type from the unit header."] fn parse_unit_type < R : Reader > (input : & mut R) -> Result < constants :: DwUt > { let val = input . read_u8 () ? ; Ok (constants :: DwUt (val)) }
};
}
