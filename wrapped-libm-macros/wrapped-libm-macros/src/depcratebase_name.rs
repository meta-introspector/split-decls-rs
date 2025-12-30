// Generated macro for base_name (function)
macro_rules! Depcratebase_name {
() => {
// Module: crate
// Provides: {"base_name"}
// Dependencies: {}
# [doc = " Return the unsuffixed version of a function name; e.g. `abs` and `absf` both return `abs`,"] # [doc = " `lgamma_r` and `lgammaf_r` both return `lgamma_r`."] fn base_name (name : & str) -> & str { let known_mappings = & [("erff" , "erf") , ("erf" , "erf") , ("lgammaf_r" , "lgamma_r") , ("modff" , "modf") , ("modf" , "modf") ,] ; match known_mappings . iter () . find (| known | known . 0 == name) { Some (found) => found . 1 , None => name . strip_suffix ("f") . or_else (| | name . strip_suffix ("f16")) . or_else (| | name . strip_suffix ("f128")) . unwrap_or (name) , } }
};
}
