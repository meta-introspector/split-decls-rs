// Generated macro for read_map_data (function)
macro_rules! Depcrate_decode_valueread_map_data {
() => {
// Module: crate::decode::value
// Provides: {"read_map_data"}
// Dependencies: {}
fn read_map_data < R : Read > (rd : & mut R , mut len : usize , depth : u16) -> Result < Vec < (Value , Value) > , Error > { let depth = super :: decrement_depth (depth) ? ; let mut vec = Vec :: new () ; while len > 0 { vec . push ((read_value_inner (rd , depth) ? , read_value_inner (rd , depth) ?)) ; len -= 1 ; } Ok (vec) }
};
}
