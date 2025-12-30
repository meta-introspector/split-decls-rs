// Generated macro for read_map_data (function)
macro_rules! Depcrate_decode_value_refread_map_data {
() => {
// Module: crate::decode::value_ref
// Provides: {"read_map_data"}
// Dependencies: {}
fn read_map_data < 'a , R > (rd : & mut R , mut len : usize , depth : u16) -> Result < Vec < (ValueRef < 'a > , ValueRef < 'a >) > , Error > where R : BorrowRead < 'a > { let depth = super :: decrement_depth (depth) ? ; let mut vec = Vec :: new () ; while len > 0 { vec . push ((read_value_ref_inner (rd , depth) ? , read_value_ref_inner (rd , depth) ? ,)) ; len -= 1 ; } Ok (vec) }
};
}
