// Generated macro for cap_wincon_color (function)
macro_rules! Depcrate_winconcap_wincon_color {
() => {
// Module: crate::wincon
// Provides: {"cap_wincon_color"}
// Dependencies: {}
fn cap_wincon_color (color : anstyle :: Color) -> Option < anstyle :: AnsiColor > { match color { anstyle :: Color :: Ansi (c) => Some (c) , anstyle :: Color :: Ansi256 (c) => c . into_ansi () , anstyle :: Color :: Rgb (_) => None , } }
};
}
