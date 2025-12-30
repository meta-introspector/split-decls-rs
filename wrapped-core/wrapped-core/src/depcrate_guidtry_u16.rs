// Generated macro for try_u16 (function)
macro_rules! Depcrate_guidtry_u16 {
() => {
// Module: crate::guid
// Provides: {"try_u16"}
// Dependencies: {}
fn try_u16 (bytes : & mut core :: str :: Bytes , delimiter : bool) -> Result < u16 > { next (bytes , 4 , delimiter) . map (| value | value as u16) . ok_or_else (invalid_guid) }
};
}
