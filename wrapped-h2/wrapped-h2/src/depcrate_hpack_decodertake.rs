// Generated macro for take (function)
macro_rules! Depcrate_hpack_decodertake {
() => {
// Module: crate::hpack::decoder
// Provides: {"take"}
// Dependencies: {}
fn take (buf : & mut Cursor < & mut BytesMut > , n : usize) -> Bytes { let pos = buf . position () as usize ; let mut head = buf . get_mut () . split_to (pos + n) ; buf . set_position (0) ; head . advance (pos) ; head . freeze () }
};
}
