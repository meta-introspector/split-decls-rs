// Generated macro for color_name (function)
macro_rules! Depcratecolor_name {
() => {
// Module: crate
// Provides: {"color_name"}
// Dependencies: {}
fn color_name (prefix : & str , color : anstyle :: Color) -> String { match color { anstyle :: Color :: Ansi (color) => { let color = anstyle :: Ansi256Color :: from_ansi (color) ; let index = color . index () as usize ; let name = ANSI_NAMES [index] ; format ! ("{prefix}-{name}") } anstyle :: Color :: Ansi256 (color) => { let index = color . index () ; format ! ("{prefix}-ansi256-{index:03}") } anstyle :: Color :: Rgb (color) => { let anstyle :: RgbColor (r , g , b) = color ; format ! ("{prefix}-rgb-{r:02X}{g:02X}{b:02X}") } } }
};
}
