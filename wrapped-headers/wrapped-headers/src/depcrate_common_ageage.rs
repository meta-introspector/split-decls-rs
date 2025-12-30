// Generated macro for Age (struct)
macro_rules! Depcrate_common_ageAge {
() => {
// Module: crate::common::age
// Provides: {"Age"}
// Dependencies: {}
# [doc = " `Age` header, defined in [RFC7234](https://tools.ietf.org/html/rfc7234#section-5.1)"] # [doc = ""] # [doc = " The \"Age\" header field conveys the sender's estimate of the amount of"] # [doc = " time since the response was generated or successfully validated at"] # [doc = " the origin server.  Age values are calculated as specified in"] # [doc = " [Section 4.2.3](https://tools.ietf.org/html/rfc7234#section-4.2.3)."] # [doc = ""] # [doc = " ## ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " Age = delta-seconds"] # [doc = " ```"] # [doc = ""] # [doc = " The Age field-value is a non-negative integer, representing time in"] # [doc = " seconds (see [Section 1.2.1](https://tools.ietf.org/html/rfc7234#section-1.2.1))."] # [doc = ""] # [doc = " The presence of an Age header field implies that the response was not"] # [doc = " generated or validated by the origin server for this request."] # [doc = " However, lack of an Age header field does not imply the origin was"] # [doc = " contacted, since the response might have been received from an"] # [doc = " HTTP/1.0 cache that does not implement Age."] # [doc = ""] # [doc = " ## Example values"] # [doc = ""] # [doc = " * `3600`"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::Age;"] # [doc = ""] # [doc = " let len = Age::from_secs(60);"] # [doc = " ```"] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct Age (Seconds) ;
};
}
