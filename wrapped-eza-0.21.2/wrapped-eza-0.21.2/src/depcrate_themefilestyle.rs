// Generated macro for FileStyle (trait)
macro_rules! Depcrate_themeFileStyle {
() => {
// Module: crate::theme
// Provides: {"FileStyle"}
// Dependencies: {}
# [doc = " Determine the style to paint the text for the filename part of the output."] pub trait FileStyle : Sync { # [doc = " Return the style to paint the filename text for `file` from the given"] # [doc = " `theme`."] fn get_style (& self , file : & File < '_ > , theme : & Theme) -> Option < Style > ; }
};
}
