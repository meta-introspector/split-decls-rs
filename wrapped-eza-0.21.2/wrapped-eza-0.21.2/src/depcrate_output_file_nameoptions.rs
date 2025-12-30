// Generated macro for Options (struct)
macro_rules! Depcrate_output_file_nameOptions {
() => {
// Module: crate::output::file_name
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Basically a file name factory."] # [derive (Debug , Copy , Clone)] pub struct Options { # [doc = " Whether to append file class characters to file names."] pub classify : Classify , # [doc = " Whether to prepend icon characters before file names."] pub show_icons : ShowIcons , # [doc = " How to display file names with spaces (with or without quotes)."] pub quote_style : QuoteStyle , # [doc = " Whether to make file names hyperlinks."] pub embed_hyperlinks : EmbedHyperlinks , # [doc = " Whether to display files with their absolute path."] pub absolute : Absolute , # [doc = " Whether we are in a console or redirecting the output"] pub is_a_tty : bool , }
};
}
