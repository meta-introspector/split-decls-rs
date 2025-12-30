// Generated macro for BytesMut (trait)
macro_rules! Depcrate_write_utilBytesMut {
() => {
// Module: crate::write::util
// Provides: {"BytesMut"}
// Dependencies: {}
# [doc = " A trait for mutable byte slices."] # [doc = ""] # [doc = " It provides convenience methods for `Pod` types."] pub (crate) trait BytesMut { fn write_at < T : Pod > (self , offset : usize , val : & T) -> Result < () , () > ; }
};
}
