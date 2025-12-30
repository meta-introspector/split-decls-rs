// Generated macro for category_and_type_from_event (function)
macro_rules! Depcratecategory_and_type_from_event {
() => {
// Module: crate
// Provides: {"category_and_type_from_event"}
// Dependencies: {}
pub fn category_and_type_from_event < T : Serialize > (ev : & T) -> (String , String) { let name = serde_json :: to_value (ev) . unwrap () ["name"] . to_string () . replace ("\"" , "") ; category_and_type_from_name (& name) }
};
}
