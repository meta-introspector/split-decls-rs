// Generated macro for SliceReader (struct)
macro_rules! Depcrate_deSliceReader {
() => {
// Module: crate::de
// Provides: {"SliceReader"}
// Dependencies: {}
# [doc = " XML input source that reads from a slice of bytes and can borrow from it."] # [doc = ""] # [doc = " You cannot create it, it is created automatically when you call"] # [doc = " [`Deserializer::from_str`]."] pub struct SliceReader < 'de > { reader : NsReader < & 'de [u8] > , }
};
}
