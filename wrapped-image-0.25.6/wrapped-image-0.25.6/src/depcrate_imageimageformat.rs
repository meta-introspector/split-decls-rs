// Generated macro for ImageFormat (enum)
macro_rules! Depcrate_imageImageFormat {
() => {
// Module: crate::image
// Provides: {"ImageFormat"}
// Dependencies: {}
# [doc = " An enumeration of supported image formats."] # [doc = " Not all formats support both encoding and decoding."] # [derive (Clone , Copy , PartialEq , Eq , Debug , Hash)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] # [non_exhaustive] pub enum ImageFormat { # [doc = " An Image in PNG Format"] Png , # [doc = " An Image in JPEG Format"] Jpeg , # [doc = " An Image in GIF Format"] Gif , # [doc = " An Image in WEBP Format"] WebP , # [doc = " An Image in general PNM Format"] Pnm , # [doc = " An Image in TIFF Format"] Tiff , # [doc = " An Image in TGA Format"] Tga , # [doc = " An Image in DDS Format"] Dds , # [doc = " An Image in BMP Format"] Bmp , # [doc = " An Image in ICO Format"] Ico , # [doc = " An Image in Radiance HDR Format"] Hdr , # [doc = " An Image in OpenEXR Format"] OpenExr , # [doc = " An Image in farbfeld Format"] Farbfeld , # [doc = " An Image in AVIF Format"] Avif , # [doc = " An Image in QOI Format"] Qoi , # [doc = " An Image in PCX Format"] Pcx , }
};
}
