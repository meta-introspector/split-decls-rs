// Generated macro for FileType (struct)
macro_rules! Depcrate_theme_ui_stylesFileType {
() => {
// Module: crate::theme::ui_styles
// Provides: {"FileType"}
// Dependencies: {}
# [doc = " Drawing styles based on the type of file (video, image, compressed, etc)"] # [rustfmt :: skip] # [derive (Clone , Copy , Debug , Eq , Default , PartialEq , Serialize , Deserialize)] pub struct FileType { pub image : Option < Style > , pub video : Option < Style > , pub music : Option < Style > , pub lossless : Option < Style > , pub crypto : Option < Style > , pub document : Option < Style > , pub compressed : Option < Style > , pub temp : Option < Style > , pub compiled : Option < Style > , pub build : Option < Style > , pub source : Option < Style > , }
};
}
