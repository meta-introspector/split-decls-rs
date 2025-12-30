// Generated macro for parse_hex (function)
macro_rules! Depcrate_scannersparse_hex {
() => {
// Module: crate::scanners
// Provides: {"parse_hex"}
// Dependencies: {}
fn parse_hex (bytes : & [u8] , limit : usize) -> (usize , usize) { match bytes . iter () . take (limit) . try_fold ((0 , 0usize) , | (count , acc) , c | { let mut c = * c ; let digit = if c . is_ascii_digit () { usize :: from (c - b'0') } else { c |= 0x20 ; if (b'a' ..= b'f') . contains (& c) { usize :: from (c - b'a' + 10) } else { return Err ((count , acc)) ; } } ; match acc . checked_mul (16) . and_then (| sixteen_acc | sixteen_acc . checked_add (digit)) { Some (number) => Ok ((count + 1 , number)) , None => Err ((count , acc)) , } }) { Ok (p) | Err (p) => p , } }
};
}
