// Generated macro for to_anstyle_color (function)
macro_rules! Depcrateto_anstyle_color {
() => {
// Module: crate
// Provides: {"to_anstyle_color"}
// Dependencies: {}
# [doc = " Convert highlighting color to general color"] pub fn to_anstyle_color (color : syntect :: highlighting :: Color) -> anstyle :: Color { anstyle :: RgbColor (color . r , color . g , color . b) . into () }
};
}
