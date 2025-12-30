// Generated macro for default_color_specs (function)
macro_rules! Depcrate_colordefault_color_specs {
() => {
// Module: crate::color
// Provides: {"default_color_specs"}
// Dependencies: {}
# [doc = " Returns a default set of color specifications."] # [doc = ""] # [doc = " This may change over time, but the color choices are meant to be fairly"] # [doc = " conservative that work across terminal themes."] # [doc = ""] # [doc = " Additional color specifications can be added to the list returned. More"] # [doc = " recently added specifications override previously added specifications."] pub fn default_color_specs () -> Vec < UserColorSpec > { vec ! [# [cfg (unix)] "path:fg:magenta" . parse () . unwrap () , # [cfg (windows)] "path:fg:cyan" . parse () . unwrap () , "line:fg:green" . parse () . unwrap () , "match:fg:red" . parse () . unwrap () , "match:style:bold" . parse () . unwrap () ,] }
};
}
