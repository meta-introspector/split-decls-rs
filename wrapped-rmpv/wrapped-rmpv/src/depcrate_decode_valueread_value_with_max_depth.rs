// Generated macro for read_value_with_max_depth (function)
macro_rules! Depcrate_decode_valueread_value_with_max_depth {
() => {
// Module: crate::decode::value
// Provides: {"read_value_with_max_depth"}
// Dependencies: {}
# [doc = " Attempts to read bytes from the given reader and interpret them as a [`Value`]."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return [`Error`] on any I/O error while either reading or decoding a [`Value`]."] # [doc = " All instances of [`ErrorKind::Interrupted`](io::ErrorKind) are handled by this function and the"] # [doc = " underlying operation is retried."] # [doc = ""] # [doc = " [`Error::DepthLimitExceeded`] is returned if this function recurses"] # [doc = " `max_depth` times. If the default [`MAX_DEPTH`](super::MAX_DEPTH) is sufficient or you do not"] # [doc = " need recursion depth checking for your data, consider using [`read_value`] instead."] # [inline] pub fn read_value_with_max_depth < R > (rd : & mut R , max_depth : usize) -> Result < Value , Error > where R : Read { read_value_inner (rd , max_depth . min (u16 :: MAX as usize) as u16) }
};
}
