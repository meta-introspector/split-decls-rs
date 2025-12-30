// Generated macro for remove_padding (function)
macro_rules! Depcrate_encodingremove_padding {
() => {
// Module: crate::encoding
// Provides: {"remove_padding"}
// Dependencies: {}
# [doc = " Remove padding from the provided input."] fn remove_padding (mut input : & [u8]) -> Result < & [u8] > { if input . len () % 8 != 0 { return Err (Error :: InvalidEncoding) ; } for _ in 0 .. 6 { match input . split_last () { Some ((b'=' , rest)) => input = rest , _ => break , } } Ok (input) }
};
}
