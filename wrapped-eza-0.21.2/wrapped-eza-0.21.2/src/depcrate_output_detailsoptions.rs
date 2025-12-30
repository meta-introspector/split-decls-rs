// Generated macro for Options (struct)
macro_rules! Depcrate_output_detailsOptions {
() => {
// Module: crate::output::details
// Provides: {"Options"}
// Dependencies: {}
# [doc = " With the **Details** view, the output gets formatted into columns, with"] # [doc = " each `Column` object showing some piece of information about the file,"] # [doc = " such as its size, or its permissions."] # [doc = ""] # [doc = " To do this, the results have to be written to a table, instead of"] # [doc = " displaying each file immediately. Then, the width of each column can be"] # [doc = " calculated based on the individual results, and the fields are padded"] # [doc = " during output."] # [doc = ""] # [doc = " Almost all the heavy lifting is done in a Table object, which handles the"] # [doc = " columns for each row."] # [allow (clippy :: struct_excessive_bools)] # [doc = " This clearly isn't a state machine"] # [derive (PartialEq , Eq , Debug)] pub struct Options { # [doc = " Options specific to drawing a table."] # [doc = ""] # [doc = " Directories themselves can pick which columns are *added* to this"] # [doc = " list, such as the Git column."] pub table : Option < TableOptions > , # [doc = " Whether to show a header line or not."] pub header : bool , # [doc = " Whether to show each file’s extended attributes."] pub xattr : bool , # [doc = " Whether to show each file's security attribute."] pub secattr : bool , # [doc = " Whether to show a directory's mounted filesystem details"] pub mounts : bool , pub color_scale : ColorScaleOptions , # [doc = " Whether to drill down into symbolic links that point to directories"] pub follow_links : bool , }
};
}
