// Generated macro for read_array_data (function)
macro_rules! Depcrate_decode_value_refread_array_data {
() => {
// Module: crate::decode::value_ref
// Provides: {"read_array_data"}
// Dependencies: {}
fn read_array_data < 'a , R > (rd : & mut R , mut len : usize , depth : u16) -> Result < Vec < ValueRef < 'a > > , Error > where R : BorrowRead < 'a > { let depth = super :: decrement_depth (depth) ? ; let mut vec = Vec :: new () ; while len > 0 { vec . push (read_value_ref_inner (rd , depth) ?) ; len -= 1 ; } Ok (vec) }
};
}
