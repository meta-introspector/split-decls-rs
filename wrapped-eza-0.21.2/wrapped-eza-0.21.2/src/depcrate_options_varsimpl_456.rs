// Generated macro for impl_456 (impl)
macro_rules! Depcrate_options_varsimpl_456 {
() => {
// Module: crate::options::vars
// Provides: {"impl_456"}
// Dependencies: {}
# [cfg (test)] # [allow (dead_code)] impl Vars for MockVars { fn get (& self , name : & 'static str) -> Option < OsString > { match name { "EXA_STRICT" | "EZA_STRICT" => Some (self . strict . clone ()) , "EZA_COLORS" | "LS_COLORS" | "EXA_COLORS" => Some (self . colors . clone ()) , "EXA_DEBUG" | "EZA_DEBUG" => Some (self . debug . clone ()) , "EXA_GRID_ROWS" | "EZA_GRID_ROWS" => Some (self . grid_rows . clone ()) , "EXA_ICON_SPACING" | "EZA_ICON_SPACING" => Some (self . icon_spacing . clone ()) , "EXA_MIN_LUMINANCE" | "EZA_MIN_LUMINANCE" => Some (self . luminance . clone ()) , "EZA_ICONS_AUTO" => Some (self . icons . clone ()) , "COLUMNS" => Some (self . columns . clone ()) , "NO_COLOR" => Some (self . no_colors . clone ()) , _ => None , } } }
};
}
