// Generated macro for icon_for_file (function)
macro_rules! Depcrate_output_iconsicon_for_file {
() => {
// Module: crate::output::icons
// Provides: {"icon_for_file"}
// Dependencies: {}
# [doc = " Lookup the icon for a file based on the file's name, if the entry is a"] # [doc = " directory, or by the lowercase file extension."] pub fn icon_for_file (file : & File < '_ >) -> char { if file . points_to_directory () { * DIRECTORY_ICONS . get (file . name . as_str ()) . unwrap_or_else (| | { if file . is_empty_dir () { & Icons :: FOLDER_OPEN } else { & Icons :: FOLDER } }) } else if let Some (icon) = FILENAME_ICONS . get (file . name . as_str ()) { * icon } else if let Some (ext) = file . ext . as_ref () { * EXTENSION_ICONS . get (ext . as_str ()) . unwrap_or (& Icons :: FILE) } else { Icons :: FILE_OUTLINE } }
};
}
