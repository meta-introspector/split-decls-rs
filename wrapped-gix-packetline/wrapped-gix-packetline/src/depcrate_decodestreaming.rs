// Generated macro for streaming (function)
macro_rules! Depcrate_decodestreaming {
() => {
// Module: crate::decode
// Provides: {"streaming"}
// Dependencies: {}
# [doc = " Decode `data` as packet line while reporting whether the data is complete or not using a [`Stream`]."] pub fn streaming (data : & [u8]) -> Result < Stream < '_ > , Error > { let data_len = data . len () ; if data_len < U16_HEX_BYTES { return Ok (Stream :: Incomplete { bytes_needed : U16_HEX_BYTES - data_len , }) ; } let wanted_bytes = match hex_prefix (& data [.. U16_HEX_BYTES]) ? { PacketLineOrWantedSize :: Wanted (s) => s as usize , PacketLineOrWantedSize :: Line (line) => { return Ok (Stream :: Complete { line , bytes_consumed : 4 , }) } } + U16_HEX_BYTES ; if wanted_bytes > MAX_LINE_LEN { return Err (Error :: DataLengthLimitExceeded { length_in_bytes : wanted_bytes , }) ; } if data_len < wanted_bytes { return Ok (Stream :: Incomplete { bytes_needed : wanted_bytes - data_len , }) ; } Ok (Stream :: Complete { line : to_data_line (& data [U16_HEX_BYTES .. wanted_bytes]) ? , bytes_consumed : wanted_bytes , }) }
};
}
