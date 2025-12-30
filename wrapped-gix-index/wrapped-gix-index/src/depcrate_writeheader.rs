// Generated macro for header (function)
macro_rules! Depcrate_writeheader {
() => {
// Module: crate::write
// Provides: {"header"}
// Dependencies: {}
fn header < T : std :: io :: Write > (out : & mut CountBytes < T > , version : Version , num_entries : u32 ,) -> Result < u32 , std :: io :: Error > { let version = match version { Version :: V2 => 2_u32 . to_be_bytes () , Version :: V3 => 3_u32 . to_be_bytes () , Version :: V4 => 4_u32 . to_be_bytes () , } ; out . write_all (crate :: decode :: header :: SIGNATURE) ? ; out . write_all (& version) ? ; out . write_all (& num_entries . to_be_bytes ()) ? ; Ok (out . count) }
};
}
