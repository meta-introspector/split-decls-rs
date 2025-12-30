// Generated macro for image_dimensions (function)
macro_rules! Depcrate_dynimageimage_dimensions {
() => {
// Module: crate::dynimage
// Provides: {"image_dimensions"}
// Dependencies: {}
# [doc = " Read a tuple containing the (width, height) of the image located at the specified path."] # [doc = " This is faster than fully loading the image and then getting its dimensions."] # [doc = ""] # [doc = " Try [`ImageReader`] for more advanced uses, including guessing the format based on the file's"] # [doc = " content before its path or manually supplying the format."] pub fn image_dimensions < P > (path : P) -> ImageResult < (u32 , u32) > where P : AsRef < Path > , { ImageReader :: open (path) ? . into_dimensions () }
};
}
