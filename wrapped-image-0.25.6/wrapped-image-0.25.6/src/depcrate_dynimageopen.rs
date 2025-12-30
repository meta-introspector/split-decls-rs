// Generated macro for open (function)
macro_rules! Depcrate_dynimageopen {
() => {
// Module: crate::dynimage
// Provides: {"open"}
// Dependencies: {}
# [doc = " Open the image located at the path specified."] # [doc = " The image's format is determined from the path's file extension."] # [doc = ""] # [doc = " Try [`ImageReader`] for more advanced uses, including guessing the format based on the file's"] # [doc = " content before its path."] pub fn open < P > (path : P) -> ImageResult < DynamicImage > where P : AsRef < Path > , { ImageReader :: open (path) ? . decode () }
};
}
