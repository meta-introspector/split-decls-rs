// Generated macro for HEX_TABLE (const)
macro_rules! Depcrate_parserHEX_TABLE {
() => {
// Module: crate::parser
// Provides: {"HEX_TABLE"}
// Dependencies: {}
const HEX_TABLE : & [u8 ; 256] = & { let mut buf = [0 ; 256] ; let mut i : u8 = 0 ; loop { buf [i as usize] = match i { b'0' ..= b'9' => i - b'0' , b'a' ..= b'f' => i - b'a' + 10 , b'A' ..= b'F' => i - b'A' + 10 , _ => 0xff , } ; if i == 255 { break buf ; } i += 1 } } ;
};
}
