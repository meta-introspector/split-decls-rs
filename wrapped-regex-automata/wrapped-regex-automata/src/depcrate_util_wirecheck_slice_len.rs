// Generated macro for check_slice_len (function)
macro_rules! Depcrate_util_wirecheck_slice_len {
() => {
// Module: crate::util::wire
// Provides: {"check_slice_len"}
// Dependencies: {}
# [doc = " Checks that the given slice has some minimal length. If it's smaller than"] # [doc = " the bound given, then a \"buffer too small\" error is returned with `what`"] # [doc = " describing what the buffer represents."] pub (crate) fn check_slice_len < T > (slice : & [T] , at_least_len : usize , what : & 'static str ,) -> Result < () , DeserializeError > { if slice . len () < at_least_len { return Err (DeserializeError :: buffer_too_small (what)) ; } Ok (()) }
};
}
