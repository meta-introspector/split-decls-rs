// Generated macro for package_metadata_value (function)
macro_rules! Depcratepackage_metadata_value {
() => {
// Module: crate
// Provides: {"package_metadata_value"}
// Dependencies: {}
fn package_metadata_value < 'a > (manifest : & 'a Value , full_key : & str) -> & 'a Value { let error_message = format ! ("Key `package.metadata.{full_key}` must be present in the Cargo manifest") ; manifest . get ("package") . and_then (| package | package . get ("metadata")) . and_then (| metadata | { let mut table = metadata . as_table () . expect ("TOML property `package.metadata` must be a table") ; let mut value = None ; for key in full_key . split ('.') { match table . get (key) . expect (& error_message) { Value :: Table (t) => { table = t ; } v => { value = Some (v) ; } } } value }) . expect (& error_message) }
};
}
