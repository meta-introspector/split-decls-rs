// Generated macro for impl_457 (impl)
macro_rules! Depcrate_options_varsimpl_457 {
() => {
// Module: crate::options::vars
// Provides: {"impl_457"}
// Dependencies: {}
# [cfg (test)] # [allow (dead_code)] impl MockVars { pub fn set (& mut self , var : & 'static str , value : & OsString) { match var { "EXA_STRICT" | "EZA_STRICT" => self . strict = value . clone () , "EZA_COLORS" | "LS_COLORS" | "EXA_COLORS" => self . colors = value . clone () , "EXA_DEBUG" | "EZA_DEBUG" => self . debug = value . clone () , "EXA_GRID_ROWS" | "EZA_GRID_ROWS" => self . grid_rows = value . clone () , "EXA_ICON_SPACING" | "EZA_ICON_SPACING" => self . icon_spacing = value . clone () , "EXA_MIN_LUMINANCE" | "EZA_MIN_LUMINANCE" => self . luminance = value . clone () , "EZA_ICONS_AUTO" => self . icons = value . clone () , "COLUMNS" => self . columns = value . clone () , "NO_COLOR" => self . no_colors = value . clone () , _ => () , } ; } }
};
}
