// Generated macro for AF_INET (const)
macro_rules! Depcrate_pg_types_network_addressAF_INET {
() => {
// Module: crate::pg::types::network_address
// Provides: {"AF_INET"}
// Dependencies: {}
# [allow (clippy :: cast_possible_truncation)] # [cfg (not (any (windows , target_os = "redox")))] const AF_INET : u8 = libc :: AF_INET as u8 ;
};
}
