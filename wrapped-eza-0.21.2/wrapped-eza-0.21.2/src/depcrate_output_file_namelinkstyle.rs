// Generated macro for LinkStyle (enum)
macro_rules! Depcrate_output_file_nameLinkStyle {
() => {
// Module: crate::output::file_name
// Provides: {"LinkStyle"}
// Dependencies: {}
# [doc = " When displaying a file name, there needs to be some way to handle broken"] # [doc = " links, depending on how long the resulting Cell can be."] # [derive (PartialEq , Debug , Copy , Clone)] enum LinkStyle { # [doc = " Just display the file names, but colour them differently if they’re"] # [doc = " a broken link or can’t be followed."] JustFilenames , # [doc = " Display all files in their usual style, but follow each link with an"] # [doc = " arrow pointing to their path, colouring the path differently if it’s"] # [doc = " a broken link, and doing nothing if it can’t be followed."] FullLinkPaths , }
};
}
