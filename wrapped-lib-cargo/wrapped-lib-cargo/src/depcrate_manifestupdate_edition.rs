// Generated macro for update_edition (function)
macro_rules! Depcrate_manifestupdate_edition {
() => {
// Module: crate::manifest
// Provides: {"update_edition"}
// Dependencies: {}
# [doc = " Update manifest edition"] pub fn update_edition (manifest : & mut Value , edition : & str) -> Result < () > { if let Some (package) = manifest . get_mut ("package") { if let Some (package_table) = package . as_table_mut () { package_table . insert ("edition" . to_string () , Value :: String (edition . to_string ())) ; } } Ok (()) }
};
}
