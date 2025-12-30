// Generated macro for QpackHeaderBlockRepresentation (enum)
macro_rules! Depcrate_events_qpackQpackHeaderBlockRepresentation {
() => {
// Module: crate::events::qpack
// Provides: {"QpackHeaderBlockRepresentation"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] pub enum QpackHeaderBlockRepresentation { IndexedHeaderField { header_field_type : QpackHeaderBlockRepresentationTypeName , table_type : QpackTableType , index : u64 , is_post_base : Option < bool > , } , LiteralHeaderFieldWithName { header_field_type : QpackHeaderBlockRepresentationTypeName , preserve_literal : bool , table_type : QpackTableType , name_index : u64 , huffman_encoded_value : bool , value_length : u64 , value : String , is_post_base : Option < bool > , } , LiteralHeaderFieldWithoutName { header_field_type : QpackHeaderBlockRepresentationTypeName , preserve_literal : bool , table_type : QpackTableType , name_index : u64 , huffman_encoded_name : bool , name_length : u64 , name : String , huffman_encoded_value : bool , value_length : u64 , value : String , is_post_base : Option < bool > , } , }
};
}
