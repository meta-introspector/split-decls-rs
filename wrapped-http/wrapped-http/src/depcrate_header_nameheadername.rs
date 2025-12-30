// Generated macro for HeaderName (struct)
macro_rules! Depcrate_header_nameHeaderName {
() => {
// Module: crate::header::name
// Provides: {"HeaderName"}
// Dependencies: {}
# [doc = " Represents an HTTP header field name"] # [doc = ""] # [doc = " Header field names identify the header. Header sets may include multiple"] # [doc = " headers with the same name. The HTTP specification defines a number of"] # [doc = " standard headers, but HTTP messages may include non-standard header names as"] # [doc = " well as long as they adhere to the specification."] # [doc = ""] # [doc = " `HeaderName` is used as the [`HeaderMap`] key. Constants are available for"] # [doc = " all standard header names in the [`header`] module."] # [doc = ""] # [doc = " # Representation"] # [doc = ""] # [doc = " `HeaderName` represents standard header names using an `enum`, as such they"] # [doc = " will not require an allocation for storage. All custom header names are"] # [doc = " lower cased upon conversion to a `HeaderName` value. This avoids the"] # [doc = " overhead of dynamically doing lower case conversion during the hash code"] # [doc = " computation and the comparison operation."] # [doc = ""] # [doc = " [`HeaderMap`]: struct.HeaderMap.html"] # [doc = " [`header`]: index.html"] # [derive (Clone , Eq , PartialEq , Hash)] pub struct HeaderName { inner : Repr < Custom > , }
};
}
