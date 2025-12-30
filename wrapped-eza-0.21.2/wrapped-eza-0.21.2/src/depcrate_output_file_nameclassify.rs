// Generated macro for Classify (enum)
macro_rules! Depcrate_output_file_nameClassify {
() => {
// Module: crate::output::file_name
// Provides: {"Classify"}
// Dependencies: {}
# [doc = " Whether to append file class characters to the file names."] # [derive (PartialEq , Eq , Debug , Default , Copy , Clone)] pub enum Classify { # [doc = " Just display the file names, without any characters."] # [default] JustFilenames , # [doc = " Always add a character after the file name depending on what class of"] # [doc = " file it is."] AddFileIndicators , AutomaticAddFileIndicators , }
};
}
