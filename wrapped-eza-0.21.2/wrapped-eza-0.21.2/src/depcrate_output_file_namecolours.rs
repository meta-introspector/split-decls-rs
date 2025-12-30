// Generated macro for Colours (trait)
macro_rules! Depcrate_output_file_nameColours {
() => {
// Module: crate::output::file_name
// Provides: {"Colours"}
// Dependencies: {}
# [doc = " The set of colours that are needed to paint a file name."] pub trait Colours : FiletypeColours { # [doc = " The style to paint the path of a symlink’s target, up to but not"] # [doc = " including the file’s name."] fn symlink_path (& self) -> Style ; # [doc = " The style to paint the arrow between a link and its target."] fn normal_arrow (& self) -> Style ; # [doc = " The style to paint the filenames of broken links in views that don’t"] # [doc = " show link targets, and the style to paint the *arrow* between the link"] # [doc = " and its target in views that *do* show link targets."] fn broken_symlink (& self) -> Style ; # [doc = " The style to paint the entire filename of a broken link."] fn broken_filename (& self) -> Style ; # [doc = " The style to paint a non-displayable control character in a filename."] fn control_char (& self) -> Style ; # [doc = " The style to paint a non-displayable control character in a filename,"] # [doc = " when the filename is being displayed as a broken link target."] fn broken_control_char (& self) -> Style ; # [doc = " The style to paint a file that has its executable bit set."] fn executable_file (& self) -> Style ; # [doc = " The style to paint a directory that has a filesystem mounted on it."] fn mount_point (& self) -> Style ; fn colour_file (& self , file : & File < '_ >) -> Style ; fn style_override (& self , file : & File < '_ >) -> Option < FileNameStyle > ; }
};
}
