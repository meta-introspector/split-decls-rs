// Generated macro for FileName (struct)
macro_rules! Depcrate_output_file_nameFileName {
() => {
// Module: crate::output::file_name
// Provides: {"FileName"}
// Dependencies: {}
# [doc = " A **file name** holds all the information necessary to display the name"] # [doc = " of the given file. This is used in all of the views."] pub struct FileName < 'a , 'dir , C > { # [doc = " A reference to the file that we’re getting the name of."] file : & 'a File < 'dir > , # [doc = " The colours used to paint the file name and its surrounding text."] colours : & 'a C , # [doc = " The file that this file points to if it’s a link."] target : Option < FileTarget < 'dir > > , # [doc = " How to handle displaying links."] link_style : LinkStyle , pub options : Options , # [doc = " How to handle displaying a mounted filesystem."] mount_style : MountStyle , }
};
}
