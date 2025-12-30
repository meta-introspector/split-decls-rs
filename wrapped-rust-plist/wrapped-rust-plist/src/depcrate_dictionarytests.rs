// Generated macro for tests (module)
macro_rules! Depcrate_dictionarytests {
() => {
// Module: crate::dictionary
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: Dictionary ; # [test] fn from_hash_map_to_dict () { let dict : Dictionary = [("Doge" , "Shiba Inu") , ("Cheems" , "Shiba Inu") , ("Walter" , "Bull Terrier") , ("Perro" , "Golden Retriever") ,] . into_iter () . collect () ; assert_eq ! (dict . get ("Doge") . and_then (| v | v . as_string ()) , Some ("Shiba Inu")) ; assert_eq ! (dict . get ("Cheems") . and_then (| v | v . as_string ()) , Some ("Shiba Inu")) ; assert_eq ! (dict . get ("Walter") . and_then (| v | v . as_string ()) , Some ("Bull Terrier")) ; assert_eq ! (dict . get ("Perro") . and_then (| v | v . as_string ()) , Some ("Golden Retriever")) ; } }
};
}
