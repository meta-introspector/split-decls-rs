// Generated macro for detect_snapshot_name (function)
macro_rules! Depcrate_runtimedetect_snapshot_name {
() => {
// Module: crate::runtime
// Provides: {"detect_snapshot_name"}
// Dependencies: {}
fn detect_snapshot_name (function_name : & str , module_path : & str) -> Result < String , & 'static str > { let name = function_name . rsplit ("::") . next () . unwrap () ; let (name , test_prefixed) = if let Some (stripped) = name . strip_prefix ("test_") { (stripped , true) } else { (name , false) } ; let name = add_suffix_to_snapshot_name (Cow :: Borrowed (name)) ; let key = format ! ("{}::{}" , module_path . replace ("::" , "__") , name) ; let mut name_clash_detection = TEST_NAME_CLASH_DETECTION . lock () . unwrap_or_else (| x | x . into_inner ()) ; match name_clash_detection . get (& key) { None => { name_clash_detection . insert (key . clone () , test_prefixed) ; } Some (& was_test_prefixed) => { if was_test_prefixed != test_prefixed { panic ! ("Insta snapshot name clash detected between '{name}' \
                     and 'test_{name}' in '{module_path}'. Rename one function.") ; } } } if allow_duplicates () { return Ok (name . to_string ()) ; } let mut counters = TEST_NAME_COUNTERS . lock () . unwrap_or_else (| x | x . into_inner ()) ; let test_idx = counters . get (& key) . cloned () . unwrap_or (0) + 1 ; let rv = if test_idx == 1 { name . to_string () } else { format ! ("{name}-{test_idx}") } ; counters . insert (key , test_idx) ; Ok (rv) }
};
}
