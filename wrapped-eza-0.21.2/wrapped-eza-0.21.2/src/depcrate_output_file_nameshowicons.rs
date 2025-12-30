// Generated macro for ShowIcons (enum)
macro_rules! Depcrate_output_file_nameShowIcons {
() => {
// Module: crate::output::file_name
// Provides: {"ShowIcons"}
// Dependencies: {}
# [doc = " Whether and how to show icons."] # [derive (PartialEq , Debug , Copy , Clone)] pub enum ShowIcons { # [doc = " Display icons next to file names, with the given number of spaces between"] # [doc = " the icon and the file name, even when output isn’t going to a terminal."] Always (u32) , # [doc = " Same as Always, but only when output is going to a terminal, not otherwise."] Automatic (u32) , # [doc = " Never display them, even when output is going to a terminal."] Never , }
};
}
