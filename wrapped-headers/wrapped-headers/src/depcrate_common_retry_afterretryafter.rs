// Generated macro for RetryAfter (struct)
macro_rules! Depcrate_common_retry_afterRetryAfter {
() => {
// Module: crate::common::retry_after
// Provides: {"RetryAfter"}
// Dependencies: {}
# [doc = " The `Retry-After` header."] # [doc = ""] # [doc = " The `Retry-After` response-header field can be used with a 503 (Service"] # [doc = " Unavailable) response to indicate how long the service is expected to be"] # [doc = " unavailable to the requesting client. This field MAY also be used with any"] # [doc = " 3xx (Redirection) response to indicate the minimum time the user-agent is"] # [doc = " asked wait before issuing the redirected request. The value of this field"] # [doc = " can be either an HTTP-date or an integer number of seconds (in decimal)"] # [doc = " after the time of the response."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " use std::time::{Duration, SystemTime};"] # [doc = " use headers::RetryAfter;"] # [doc = ""] # [doc = " let delay = RetryAfter::delay(Duration::from_secs(300));"] # [doc = " let date = RetryAfter::date(SystemTime::now());"] # [doc = " ```"] # [doc = ""] # [doc = " Retry-After header, defined in [RFC7231](https://datatracker.ietf.org/doc/html/rfc7231#section-7.1.3)"] # [derive (Debug , Clone , PartialEq , Eq)] pub struct RetryAfter (After) ;
};
}
