// Generated macro for ImageFormatHint (enum)
macro_rules! Depcrate_errorImageFormatHint {
() => {
// Module: crate::error
// Provides: {"ImageFormatHint"}
// Dependencies: {}
# [doc = " A best effort representation for image formats."] # [derive (Clone , Debug , Hash , PartialEq)] # [non_exhaustive] pub enum ImageFormatHint { # [doc = " The format is known exactly."] Exact (ImageFormat) , # [doc = " The format can be identified by a name."] Name (String) , # [doc = " A common path extension for the format is known."] PathExtension (std :: path :: PathBuf) , # [doc = " The format is not known or could not be determined."] Unknown , }
};
}
