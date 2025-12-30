// Generated macro for to_data_line (function)
macro_rules! Depcrate_decodeto_data_line {
() => {
// Module: crate::decode
// Provides: {"to_data_line"}
// Dependencies: {}
# [doc = " Obtain a `PacketLine` from `data` after assuring `data` is small enough to fit."] pub fn to_data_line (data : & [u8]) -> Result < PacketLineRef < '_ > , Error > { if data . len () > MAX_LINE_LEN { return Err (Error :: DataLengthLimitExceeded { length_in_bytes : data . len () , }) ; } Ok (PacketLineRef :: Data (data)) }
};
}
