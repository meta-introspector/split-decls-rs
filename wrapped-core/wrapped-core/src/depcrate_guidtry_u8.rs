// Generated macro for try_u8 (function)
macro_rules! Depcrate_guidtry_u8 {
() => {
// Module: crate::guid
// Provides: {"try_u8"}
// Dependencies: {}
fn try_u8 (bytes : & mut core :: str :: Bytes , delimiter : bool) -> Result < u8 > { next (bytes , 2 , delimiter) . map (| value | value as u8) . ok_or_else (invalid_guid) }
};
}
