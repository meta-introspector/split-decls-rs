// Generated macro for hex_decode (function)
macro_rules! Depcrate_encodinghex_decode {
() => {
// Module: crate::encoding
// Provides: {"hex_decode"}
// Dependencies: {}
pub fn hex_decode (v : & str) -> Option < Vec < u8 > > { if v . len () % 2 != 0 { return None ; } let mut b = Vec :: with_capacity (v . len () / 2) ; let v = v . as_bytes () ; for i in (0 .. v . len ()) . step_by (2) { let high = match v [i] { b @ b'0' ..= b'9' => b - b'0' , b @ b'a' ..= b'f' => b - b'a' + 10 , b @ b'A' ..= b'F' => b - b'A' + 10 , _ => return None , } ; let low = match v [i + 1] { b @ b'0' ..= b'9' => b - b'0' , b @ b'a' ..= b'f' => b - b'a' + 10 , b @ b'A' ..= b'F' => b - b'A' + 10 , _ => return None , } ; b . push ((high << 4) | low) ; } Some (b) }
};
}
