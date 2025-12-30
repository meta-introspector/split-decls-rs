// Generated macro for ascii (module)
macro_rules! Depcrate_serascii {
() => {
// Module: crate::ser
// Provides: {"ascii"}
// Dependencies: {}
# [doc = " Module is available if borsh is built with `features = [\"ascii\"]`."] # [cfg (feature = "ascii")] pub mod ascii { # ! [doc = ""] # ! [doc = " Module defines [BorshSerialize] implementation for"] # ! [doc = " some types from [ascii](::ascii) crate."] use super :: BorshSerialize ; use crate :: io :: { Result , Write } ; impl BorshSerialize for ascii :: AsciiChar { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { self . as_byte () . serialize (writer) } } impl BorshSerialize for ascii :: AsciiStr { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { self . as_bytes () . serialize (writer) } } impl BorshSerialize for ascii :: AsciiString { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { self . as_bytes () . serialize (writer) } } }
};
}
