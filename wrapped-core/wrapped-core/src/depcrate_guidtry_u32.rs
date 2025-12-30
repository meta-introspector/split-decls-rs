// Generated macro for try_u32 (function)
macro_rules! Depcrate_guidtry_u32 {
() => {
// Module: crate::guid
// Provides: {"try_u32"}
// Dependencies: {}
fn try_u32 (bytes : & mut core :: str :: Bytes , delimiter : bool) -> Result < u32 > { next (bytes , 8 , delimiter) . ok_or_else (invalid_guid) }
};
}
