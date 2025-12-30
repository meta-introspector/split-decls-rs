// Generated macro for parse_decimal (function)
macro_rules! Depcrate_scannersparse_decimal {
() => {
// Module: crate::scanners
// Provides: {"parse_decimal"}
// Dependencies: {}
fn parse_decimal (bytes : & [u8] , limit : usize) -> (usize , usize) { match bytes . iter () . take (limit) . take_while (| & & b | is_digit (b)) . try_fold ((0 , 0usize) , | (count , acc) , c | { let digit = usize :: from (c - b'0') ; match acc . checked_mul (10) . and_then (| ten_acc | ten_acc . checked_add (digit)) { Some (number) => Ok ((count + 1 , number)) , None => Err ((count , acc)) , } }) { Ok (p) | Err (p) => p , } }
};
}
