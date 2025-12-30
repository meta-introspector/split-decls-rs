// Generated macro for ascii (module)
macro_rules! Depcrate_schemaascii {
() => {
// Module: crate::schema
// Provides: {"ascii"}
// Dependencies: {}
# [doc = " Module is available if borsh is built with `features = [\"ascii\"]`."] # [cfg (feature = "ascii")] pub mod ascii { # ! [doc = ""] # ! [doc = " Module defines [BorshSchema] implementation for"] # ! [doc = " some types from [ascii](::ascii) crate."] use crate :: BorshSchema ; use super :: { add_definition , Declaration , Definition } ; use crate :: __private :: maybestd :: collections :: BTreeMap ; impl BorshSchema for ascii :: AsciiString { # [inline] fn add_definitions_recursively (definitions : & mut BTreeMap < Declaration , Definition >) { ascii :: AsciiStr :: add_definitions_recursively (definitions) ; } # [inline] fn declaration () -> Declaration { ascii :: AsciiStr :: declaration () } } impl BorshSchema for ascii :: AsciiStr { # [inline] fn add_definitions_recursively (definitions : & mut BTreeMap < Declaration , Definition >) { let definition = Definition :: Sequence { length_width : Definition :: DEFAULT_LENGTH_WIDTH , length_range : Definition :: DEFAULT_LENGTH_RANGE , elements : ascii :: AsciiChar :: declaration () , } ; add_definition (Self :: declaration () , definition , definitions) ; ascii :: AsciiChar :: add_definitions_recursively (definitions) ; } # [inline] fn declaration () -> Declaration { "AsciiString" . into () } } impl BorshSchema for ascii :: AsciiChar { # [inline] fn add_definitions_recursively (definitions : & mut BTreeMap < Declaration , Definition >) { add_definition (Self :: declaration () , Definition :: Primitive (1) , definitions) ; } # [inline] fn declaration () -> Declaration { "AsciiChar" . into () } } }
};
}
