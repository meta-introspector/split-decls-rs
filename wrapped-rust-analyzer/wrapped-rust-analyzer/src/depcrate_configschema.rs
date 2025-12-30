// Generated macro for schema (function)
macro_rules! Depcrate_configschema {
() => {
// Module: crate::config
// Provides: {"schema"}
// Dependencies: {}
fn schema (fields : & [SchemaField]) -> serde_json :: Value { let map = fields . iter () . map (| (field , ty , doc , default) | { let name = field . replace ('_' , ".") ; let category = name . split_once (".") . map (| (category , _name) | to_title_case (category)) . unwrap_or ("rust-analyzer" . into ()) ; let name = format ! ("rust-analyzer.{name}") ; let props = field_props (field , ty , doc , default) ; serde_json :: json ! ({ "title" : category , "properties" : { name : props } }) }) . collect :: < Vec < _ > > () ; map . into () }
};
}
