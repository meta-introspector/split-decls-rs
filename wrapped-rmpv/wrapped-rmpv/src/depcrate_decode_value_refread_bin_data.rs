// Generated macro for read_bin_data (function)
macro_rules! Depcrate_decode_value_refread_bin_data {
() => {
// Module: crate::decode::value_ref
// Provides: {"read_bin_data"}
// Dependencies: {}
fn read_bin_data < 'a , R > (rd : & mut R , len : usize , depth : u16) -> Result < & 'a [u8] , Error > where R : BorrowRead < 'a > { let _depth = super :: decrement_depth (depth) ? ; let buf = rd . fill_buf () ; if len > buf . len () { return Err (Error :: InvalidDataRead (io :: Error :: new (ErrorKind :: UnexpectedEof , "unexpected EOF"))) ; } let buf = & buf [.. len] ; rd . consume (len) ; Ok (buf) }
};
}
