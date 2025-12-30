// Generated macro for check_cfg_arg (function)
macro_rules! Depcrate_utils_helperscheck_cfg_arg {
() => {
// Module: crate::utils::helpers
// Provides: {"check_cfg_arg"}
// Dependencies: {}
# [doc = " Create a `--check-cfg` argument invocation for a given name"] # [doc = " and it's values."] pub fn check_cfg_arg (name : & str , values : Option < & [& str] >) -> String { let next = match values { Some (values) => { let mut tmp = values . iter () . flat_map (| val | ["," , "\"" , val , "\""]) . collect :: < String > () ; tmp . insert_str (1 , "values(") ; tmp . push (')') ; tmp } None => "" . to_string () , } ; format ! ("--check-cfg=cfg({name}{next})") }
};
}
