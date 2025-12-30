// Generated macro for Options (struct)
macro_rules! Depcrate_optionsOptions {
() => {
// Module: crate::options
// Provides: {"Options"}
// Dependencies: {}
# [doc = " These **options** represent a parsed, error-checked versions of the"] # [doc = " user’s command-line options."] # [derive (Debug)] pub struct Options { # [doc = " The action to perform when encountering a directory rather than a"] # [doc = " regular file."] pub dir_action : DirAction , # [doc = " How to sort and filter files before outputting them."] pub filter : FileFilter , # [doc = " The user’s preference of view to use (lines, grid, details, or"] # [doc = " grid-details) along with the options on how to render file names."] # [doc = " If the view requires the terminal to have a width, and there is no"] # [doc = " width, then the view will be downgraded."] pub view : View , # [doc = " The options to make up the styles of the UI and file names."] pub theme : ThemeOptions , # [doc = " Whether to read file names from stdin instead of the command-line"] pub stdin : FilesInput , }
};
}
