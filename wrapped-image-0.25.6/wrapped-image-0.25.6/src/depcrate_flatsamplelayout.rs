// Generated macro for SampleLayout (struct)
macro_rules! Depcrate_flatSampleLayout {
() => {
// Module: crate::flat
// Provides: {"SampleLayout"}
// Dependencies: {}
# [doc = " A ffi compatible description of a sample buffer."] # [repr (C)] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub struct SampleLayout { # [doc = " The number of channels in the color representation of the image."] pub channels : u8 , # [doc = " Add this to an index to get to the sample in the next channel."] pub channel_stride : usize , # [doc = " The width of the represented image."] pub width : u32 , # [doc = " Add this to an index to get to the next sample in x-direction."] pub width_stride : usize , # [doc = " The height of the represented image."] pub height : u32 , # [doc = " Add this to an index to get to the next sample in y-direction."] pub height_stride : usize , }
};
}
