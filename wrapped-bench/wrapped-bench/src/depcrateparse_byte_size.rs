// Generated macro for parse_byte_size (function)
macro_rules! Depcrateparse_byte_size {
() => {
// Module: crate
// Provides: {"parse_byte_size"}
// Dependencies: {}
fn parse_byte_size (s : & str) -> Result < u64 , ParseIntError > { let s = s . trim () ; let multiplier = match s . chars () . last () { Some ('T') => 1024 * 1024 * 1024 * 1024 , Some ('G') => 1024 * 1024 * 1024 , Some ('M') => 1024 * 1024 , Some ('k') => 1024 , _ => 1 , } ; let s = match multiplier { 1 => s , _ => & s [.. s . len () - 1] , } ; Ok (u64 :: from_str (s) ? * multiplier) }
};
}
