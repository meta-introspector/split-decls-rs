// Generated macro for impl_42 (impl)
macro_rules! Depcrateimpl_42 {
() => {
// Module: crate
// Provides: {"impl_42"}
// Dependencies: {}
impl ToMoonBitTypeIdent for str { fn to_moonbit_type_ident (& self) -> String { match self . to_upper_camel_case () . as_str () { type_name @ ("Bool" | "Byte" | "Int" | "Int64" | "UInt" | "UInt64" | "Float" | "Double" | "Error" | "Buffer" | "Bytes" | "Array" | "FixedArray" | "Map" | "String" | "Option" | "Result" | "Char" | "Json") => { format ! ("{type_name}_") } type_name => type_name . to_owned () , } } }
};
}
