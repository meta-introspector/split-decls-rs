// Generated macro for parse_dwo_id (function)
macro_rules! Depcrate_read_unitparse_dwo_id {
() => {
// Module: crate::read::unit
// Provides: {"parse_dwo_id"}
// Dependencies: {}
# [doc = " Parse a dwo_id from a header"] fn parse_dwo_id < R : Reader > (input : & mut R) -> Result < DwoId > { Ok (DwoId (input . read_u64 () ?)) }
};
}
