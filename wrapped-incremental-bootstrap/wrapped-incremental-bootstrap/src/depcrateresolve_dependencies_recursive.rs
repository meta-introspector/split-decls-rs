// Generated macro for resolve_dependencies_recursive (function)
macro_rules! Depcrateresolve_dependencies_recursive {
() => {
// Module: crate
// Provides: {"resolve_dependencies_recursive"}
// Dependencies: {}
fn resolve_dependencies_recursive (content : & str , index : & Output2Index , resolved_includes : & mut String , visited : & mut HashSet < String > , depth : usize , glossary : & mut HashMap < String , String >) -> Result < () , Box < dyn std :: error :: Error > > { if depth > 3 { return Ok (()) ; } let missing_types = find_missing_types (content) ; for token in missing_types { if visited . contains (& token) { continue ; } visited . insert (token . clone ()) ; if let Some (paths) = index . resolve (& token) { if let Some (first_path) = paths . first () { let abs_path = std :: fs :: canonicalize (first_path) . unwrap_or_else (| _ | first_path . clone ()) ; println ! ("🔗 Resolving {} -> {}" , token , abs_path . display ()) ; glossary . insert (token . clone () , abs_path . to_string_lossy () . to_string ()) ; resolved_includes . push_str (& format ! ("    include!(\"{}\"); // for {}\n" , abs_path . display () , token)) ; if let Ok (dep_content) = fs :: read_to_string (first_path) { resolve_dependencies_recursive (& dep_content , index , resolved_includes , visited , depth + 1 , glossary) ? ; } } } } Ok (()) }
};
}
