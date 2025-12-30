// Generated macro for dbg_dmp (function)
macro_rules! Depcrate_errordbg_dmp {
() => {
// Module: crate::error
// Provides: {"dbg_dmp"}
// Dependencies: {}
# [doc = " Prints a message and the input if the parser fails."] # [doc = ""] # [doc = " The message prints the `Error` or `Incomplete`"] # [doc = " and the parser's calling code."] # [doc = ""] # [doc = " It also displays the input in hexdump format"] # [doc = ""] # [doc = " ```rust"] # [doc = " use nom::{IResult, error::dbg_dmp, bytes::complete::tag};"] # [doc = ""] # [doc = " fn f(i: &[u8]) -> IResult<&[u8], &[u8]> {"] # [doc = "   dbg_dmp(tag(\"abcd\"), \"tag\")(i)"] # [doc = " }"] # [doc = ""] # [doc = "   let a = &b\"efghijkl\"[..];"] # [doc = ""] # [doc = " // Will print the following message:"] # [doc = " // Error(Position(0, [101, 102, 103, 104, 105, 106, 107, 108])) at l.5 by ' tag ! ( \"abcd\" ) '"] # [doc = " // 00000000        65 66 67 68 69 6a 6b 6c         efghijkl"] # [doc = " f(a);"] # [doc = " ```"] # [cfg (feature = "std")] # [cfg_attr (feature = "docsrs" , doc (cfg (feature = "std")))] pub fn dbg_dmp < 'a , F , O , E : std :: fmt :: Debug > (f : F , context : & 'static str ,) -> impl Fn (& 'a [u8]) -> IResult < & 'a [u8] , O , E > where F : Fn (& 'a [u8]) -> IResult < & 'a [u8] , O , E > , { use crate :: HexDisplay ; move | i : & 'a [u8] | match f (i) { Err (e) => { println ! ("{}: Error({:?}) at:\n{}" , context , e , i . to_hex (8)) ; Err (e) } a => a , } }
};
}
