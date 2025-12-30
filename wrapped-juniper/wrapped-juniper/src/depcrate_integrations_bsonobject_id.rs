// Generated macro for object_id (module)
macro_rules! Depcrate_integrations_bsonobject_id {
() => {
// Module: crate::integrations::bson
// Provides: {"object_id"}
// Dependencies: {}
mod object_id { use super :: ObjectId ; pub (super) fn to_output (v : & ObjectId) -> String { v . to_hex () } pub (super) fn from_input (s : & str) -> Result < ObjectId , Box < str > > { ObjectId :: parse_str (s) . map_err (| e | format ! ("Failed to parse `ObjectID`: {e}") . into ()) } }
};
}
