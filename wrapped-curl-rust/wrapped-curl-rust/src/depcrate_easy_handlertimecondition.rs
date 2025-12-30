// Generated macro for TimeCondition (enum)
macro_rules! Depcrate_easy_handlerTimeCondition {
() => {
// Module: crate::easy::handler
// Provides: {"TimeCondition"}
// Dependencies: {}
# [doc = " Possible conditions for the `time_condition` method."] # [non_exhaustive] # [allow (missing_docs)] # [derive (Debug , Clone , Copy)] pub enum TimeCondition { None = curl_sys :: CURL_TIMECOND_NONE as isize , IfModifiedSince = curl_sys :: CURL_TIMECOND_IFMODSINCE as isize , IfUnmodifiedSince = curl_sys :: CURL_TIMECOND_IFUNMODSINCE as isize , LastModified = curl_sys :: CURL_TIMECOND_LASTMOD as isize , }
};
}
