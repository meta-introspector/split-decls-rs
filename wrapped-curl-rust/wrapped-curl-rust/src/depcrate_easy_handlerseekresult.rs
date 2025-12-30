// Generated macro for SeekResult (enum)
macro_rules! Depcrate_easy_handlerSeekResult {
() => {
// Module: crate::easy::handler
// Provides: {"SeekResult"}
// Dependencies: {}
# [doc = " Possible return values from the `seek_function` callback."] # [non_exhaustive] # [derive (Debug , Clone , Copy)] pub enum SeekResult { # [doc = " Indicates that the seek operation was a success"] Ok = curl_sys :: CURL_SEEKFUNC_OK as isize , # [doc = " Indicates that the seek operation failed, and the entire request should"] # [doc = " fail as a result."] Fail = curl_sys :: CURL_SEEKFUNC_FAIL as isize , # [doc = " Indicates that although the seek failed libcurl should attempt to keep"] # [doc = " working if possible (for example \"seek\" through reading)."] CantSeek = curl_sys :: CURL_SEEKFUNC_CANTSEEK as isize , }
};
}
