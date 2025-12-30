// Generated macro for ascii (module)
macro_rules! Depcrate_deascii {
() => {
// Module: crate::de
// Provides: {"ascii"}
// Dependencies: {}
# [doc = " Module is available if borsh is built with `features = [\"ascii\"]`."] # [cfg (feature = "ascii")] pub mod ascii { # ! [doc = ""] # ! [doc = " Module defines [BorshDeserialize] implementation for"] # ! [doc = " some types from [ascii](::ascii) crate."] use crate :: BorshDeserialize ; use crate :: __private :: maybestd :: { string :: ToString , vec :: Vec } ; use crate :: io :: { Error , ErrorKind , Read , Result } ; impl BorshDeserialize for ascii :: AsciiString { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { let bytes = Vec :: < u8 > :: deserialize_reader (reader) ? ; ascii :: AsciiString :: from_ascii (bytes) . map_err (| err | Error :: new (ErrorKind :: InvalidData , err . to_string ())) } } impl BorshDeserialize for ascii :: AsciiChar { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { let byte = u8 :: deserialize_reader (reader) ? ; ascii :: AsciiChar :: from_ascii (byte) . map_err (| err | Error :: new (ErrorKind :: InvalidData , err . to_string ())) } } }
};
}
