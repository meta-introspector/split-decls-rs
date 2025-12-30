// Generated macro for read_value (function)
macro_rules! Depcrate_reader_tables_attributeread_value {
() => {
// Module: crate::reader::tables::attribute
// Provides: {"read_value"}
// Dependencies: {}
fn read_value (blob : & mut Blob , ty : & Type , name : & mut String) -> Value { match ty { Type :: Bool => Value :: Bool (blob . read_bool ()) , Type :: I8 => Value :: I8 (blob . read_i8 ()) , Type :: U8 => Value :: U8 (blob . read_u8 ()) , Type :: I16 => Value :: I16 (blob . read_i16 ()) , Type :: U16 => Value :: U16 (blob . read_u16 ()) , Type :: I32 => Value :: I32 (blob . read_i32 ()) , Type :: U32 => Value :: U32 (blob . read_u32 ()) , Type :: I64 => Value :: I64 (blob . read_i64 ()) , Type :: U64 => Value :: U64 (blob . read_u64 ()) , Type :: String => Value :: Utf8 (blob . read_utf8 ()) , Type :: Name (tn) => { if tn . namespace == "System" && tn . name == "Type" { Value :: Utf8 (blob . read_utf8 ()) } else { Value :: I32 (blob . read_i32 ()) } } Type :: AttributeEnum => { let enum_name = name . clone () ; * name = blob . read_utf8 () ; Value :: AttributeEnum (enum_name , blob . read_i32 ()) } rest => panic ! ("{rest:?}") , } }
};
}
