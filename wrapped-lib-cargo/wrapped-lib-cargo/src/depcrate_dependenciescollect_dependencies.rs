// Generated macro for collect_dependencies (function)
macro_rules! Depcrate_dependenciescollect_dependencies {
() => {
// Module: crate::dependencies
// Provides: {"collect_dependencies"}
// Dependencies: {}
# [doc = " Collect all dependencies from a manifest"] pub fn collect_dependencies (manifest : & Value) -> HashMap < String , Value > { let mut deps = HashMap :: new () ; for section in ["dependencies" , "dev-dependencies" , "build-dependencies"] { if let Some (section_deps) = manifest . get (section) . and_then (| v | v . as_table ()) { for (name , value) in section_deps { if let Some (table) = value . as_table () { if let Some (package_name) = table . get ("package") . and_then (| v | v . as_str ()) { deps . insert (package_name . to_string () , value . clone ()) ; } else { deps . insert (name . clone () , value . clone ()) ; } } else { deps . insert (name . clone () , value . clone ()) ; } } } } deps }
};
}
