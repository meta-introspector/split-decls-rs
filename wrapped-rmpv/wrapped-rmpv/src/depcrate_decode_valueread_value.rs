// Generated macro for read_value (function)
macro_rules! Depcrate_decode_valueread_value {
() => {
// Module: crate::decode::value
// Provides: {"read_value"}
// Dependencies: {}
# [doc = " Attempts to read bytes from the given reader and interpret them as a [`Value`]."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return [`Error`] on any I/O error while either reading or decoding a [`Value`]."] # [doc = " All instances of [`ErrorKind::Interrupted`](io::ErrorKind) are handled by this function and the"] # [doc = " underlying operation is retried."] # [doc = ""] # [doc = " [`Error::DepthLimitExceeded`] is returned if this function recurses"] # [doc = " [`MAX_DEPTH`](super::MAX_DEPTH) times. To configure the maximum recursion depth, use"] # [doc = " [`read_value_with_max_depth`] instead."] # [inline] pub fn read_value < R > (rd : & mut R) -> Result < Value , Error > where R : Read { read_value_inner (rd , super :: MAX_DEPTH as _) }
};
}
