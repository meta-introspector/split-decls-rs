// Generated macro for Render (struct)
macro_rules! Depcrate_output_grid_detailsRender {
() => {
// Module: crate::output::grid_details
// Provides: {"Render"}
// Dependencies: {}
pub struct Render < 'a > { # [doc = " The directory that’s being rendered here."] # [doc = " We need this to know which columns to put in the output."] pub dir : Option < & 'a Dir > , # [doc = " The files that have been read from the directory. They should all"] # [doc = " hold a reference to it."] pub files : Vec < File < 'a > > , # [doc = " How to colour various pieces of text."] pub theme : & 'a Theme , # [doc = " How to format filenames."] pub file_style : & 'a FileStyle , # [doc = " The details part of the grid-details view."] pub details : & 'a DetailsOptions , # [doc = " How to filter files after listing a directory. The files in this"] # [doc = " render will already have been filtered and sorted, but any directories"] # [doc = " that we recurse into will have to have this applied."] pub filter : & 'a FileFilter , # [doc = " The minimum number of rows that there need to be before grid-details"] # [doc = " mode is activated."] # [allow (dead_code)] pub row_threshold : RowThreshold , # [doc = " Whether we are skipping Git-ignored files."] pub git_ignoring : bool , pub git : Option < & 'a GitCache > , pub console_width : usize , pub git_repos : bool , }
};
}
