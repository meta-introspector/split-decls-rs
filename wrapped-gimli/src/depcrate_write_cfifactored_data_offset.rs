// Generated macro for factored_data_offset (function)
macro_rules! Depcrate_write_cfifactored_data_offset {
() => {
// Module: crate::write::cfi
// Provides: {"factored_data_offset"}
// Dependencies: {}
fn factored_data_offset (offset : i32 , factor : i8) -> Result < i32 > { let factor = i32 :: from (factor) ; let factored_offset = offset / factor ; if offset != factored_offset * factor { return Err (Error :: InvalidFrameDataOffset (offset)) ; } Ok (factored_offset) }
};
}
