// Generated macro for Render (struct)
macro_rules! Depcrate_output_detailsRender {
() => {
// Module: crate::output::details
// Provides: {"Render"}
// Dependencies: {}
pub struct Render < 'a > { pub dir : Option < & 'a Dir > , pub files : Vec < File < 'a > > , pub theme : & 'a Theme , pub file_style : & 'a FileStyle , pub opts : & 'a Options , # [doc = " Whether to recurse through directories with a tree view, and if so,"] # [doc = " which options to use. This field is only relevant here if the `tree`"] # [doc = " field of the `RecurseOptions` is `true`."] pub recurse : Option < RecurseOptions > , # [doc = " How to sort and filter the files after getting their details."] pub filter : & 'a FileFilter , # [doc = " Whether we are skipping Git-ignored files."] pub git_ignoring : bool , pub git : Option < & 'a GitCache > , pub git_repos : bool , }
};
}
