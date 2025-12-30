// Generated macro for Encoder (struct)
macro_rules! Depcrate_encoderEncoder {
() => {
// Module: crate::encoder
// Provides: {"Encoder"}
// Dependencies: {}
# [doc = " Stateful Base64 encoder with support for buffered, incremental encoding."] # [doc = ""] # [doc = " The `E` type parameter can be any type which impls [`Encoding`] such as"] # [doc = " [`Base64`] or [`Base64Unpadded`]."] pub struct Encoder < 'o , E : Encoding > { # [doc = " Output buffer."] output : & 'o mut [u8] , # [doc = " Cursor within the output buffer."] position : usize , # [doc = " Block buffer used for non-block-aligned data."] block_buffer : BlockBuffer , # [doc = " Configuration and state for line-wrapping the output at a specified"] # [doc = " column."] line_wrapper : Option < LineWrapper > , # [doc = " Phantom parameter for the Base64 encoding in use."] encoding : PhantomData < E > , }
};
}
