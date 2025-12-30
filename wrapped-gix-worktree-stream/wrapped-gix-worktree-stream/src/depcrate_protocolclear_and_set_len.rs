// Generated macro for clear_and_set_len (function)
macro_rules! Depcrate_protocolclear_and_set_len {
() => {
// Module: crate::protocol
// Provides: {"clear_and_set_len"}
// Dependencies: {}
fn clear_and_set_len (buf : & mut Vec < u8 > , len : usize) -> io :: Result < () > { buf . clear () ; buf . try_reserve (len) . map_err (| _ | ErrorKind :: OutOfMemory) ? ; buf . resize (len , 0) ; Ok (()) }
};
}
