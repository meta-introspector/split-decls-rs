// Generated macro for did_you_mean_flag (function)
macro_rules! Depcrate_parser_features_suggestionsdid_you_mean_flag {
() => {
// Module: crate::parser::features::suggestions
// Provides: {"did_you_mean_flag"}
// Dependencies: {}
# [doc = " Returns a suffix that can be empty, or is the standard 'did you mean' phrase"] pub (crate) fn did_you_mean_flag < 'a , 'help , I , T > (arg : & str , remaining_args : & [& std :: ffi :: OsStr] , longs : I , subcommands : impl IntoIterator < Item = & 'a mut Command > ,) -> Option < (String , Option < String >) > where 'help : 'a , T : AsRef < str > , I : IntoIterator < Item = T > , { use crate :: mkeymap :: KeyType ; match did_you_mean (arg , longs) . pop () { Some (candidate) => Some ((candidate , None)) , None => subcommands . into_iter () . filter_map (| subcommand | { subcommand . _build_self (false) ; let longs = subcommand . get_keymap () . keys () . filter_map (| a | { if let KeyType :: Long (v) = a { Some (v . to_string_lossy () . into_owned ()) } else { None } }) ; let subcommand_name = subcommand . get_name () ; let candidate = some ! (did_you_mean (arg , longs) . pop ()) ; let score = some ! (remaining_args . iter () . position (| x | subcommand_name == * x)) ; Some ((score , (candidate , Some (subcommand_name . to_string ())))) }) . min_by_key (| (x , _) | * x) . map (| (_ , suggestion) | suggestion) , } }
};
}
