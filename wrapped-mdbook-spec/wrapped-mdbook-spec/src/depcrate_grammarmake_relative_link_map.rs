// Generated macro for make_relative_link_map (function)
macro_rules! Depcrate_grammarmake_relative_link_map {
() => {
// Module: crate::grammar
// Provides: {"make_relative_link_map"}
// Dependencies: {}
# [doc = " Creates a map of production name -> relative link path."] fn make_relative_link_map (grammar : & Grammar , chapter : & Chapter) -> HashMap < String , String > { let current_path = chapter . path . as_ref () . unwrap () . parent () . unwrap () ; grammar . productions . values () . map (| p | { let relative = pathdiff :: diff_paths (& p . path , current_path) . unwrap () ; let relative = relative . display () . to_string () . replace ('\\' , "/") ; (p . name . clone () , relative) }) . collect () }
};
}
