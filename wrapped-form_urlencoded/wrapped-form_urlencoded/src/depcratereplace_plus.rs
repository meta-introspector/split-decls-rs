// Generated macro for replace_plus (function)
macro_rules! Depcratereplace_plus {
() => {
// Module: crate
// Provides: {"replace_plus"}
// Dependencies: {}
# [doc = " Replace b'+' with b' '"] fn replace_plus (input : & [u8]) -> Cow < '_ , [u8] > { match input . iter () . position (| & b | b == b'+') { None => Cow :: Borrowed (input) , Some (first_position) => { let mut replaced = input . to_owned () ; replaced [first_position] = b' ' ; for byte in & mut replaced [first_position + 1 ..] { if * byte == b'+' { * byte = b' ' ; } } Cow :: Owned (replaced) } } }
};
}
