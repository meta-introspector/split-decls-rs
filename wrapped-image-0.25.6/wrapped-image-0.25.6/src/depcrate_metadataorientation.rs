// Generated macro for Orientation (enum)
macro_rules! Depcrate_metadataOrientation {
() => {
// Module: crate::metadata
// Provides: {"Orientation"}
// Dependencies: {}
# [doc = " Describes the transformations to be applied to the image."] # [doc = " Compatible with [Exif orientation](https://web.archive.org/web/20200412005226/https://www.impulseadventure.com/photo/exif-orientation.html)."] # [doc = ""] # [doc = " Orientation is specified in the file's metadata, and is often written by cameras."] # [doc = ""] # [doc = " You can apply it to an image via [`DynamicImage::apply_orientation`](crate::DynamicImage::apply_orientation)."] # [derive (Copy , Clone , PartialEq , Eq , Hash , Debug)] pub enum Orientation { # [doc = " Do not perform any transformations."] NoTransforms , # [doc = " Rotate by 90 degrees clockwise."] Rotate90 , # [doc = " Rotate by 180 degrees. Can be performed in-place."] Rotate180 , # [doc = " Rotate by 270 degrees clockwise. Equivalent to rotating by 90 degrees counter-clockwise."] Rotate270 , # [doc = " Flip horizontally. Can be performed in-place."] FlipHorizontal , # [doc = " Flip vertically. Can be performed in-place."] FlipVertical , # [doc = " Rotate by 90 degrees clockwise and flip horizontally."] Rotate90FlipH , # [doc = " Rotate by 270 degrees clockwise and flip horizontally."] Rotate270FlipH , }
};
}
