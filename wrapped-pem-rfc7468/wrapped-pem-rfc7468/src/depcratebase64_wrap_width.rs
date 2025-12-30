// Generated macro for BASE64_WRAP_WIDTH (const)
macro_rules! DepcrateBASE64_WRAP_WIDTH {
() => {
// Module: crate
// Provides: {"BASE64_WRAP_WIDTH"}
// Dependencies: {}
# [doc = " Width at which the Base64 body of RFC7468-compliant PEM is wrapped."] # [doc = ""] # [doc = " From [RFC7468 § 2]:"] # [doc = ""] # [doc = " > Generators MUST wrap the base64-encoded lines so that each line"] # [doc = " > consists of exactly 64 characters except for the final line, which"] # [doc = " > will encode the remainder of the data (within the 64-character line"] # [doc = " > boundary), and they MUST NOT emit extraneous whitespace.  Parsers MAY"] # [doc = " > handle other line sizes."] # [doc = ""] # [doc = " [RFC7468 § 2]: https://datatracker.ietf.org/doc/html/rfc7468#section-2"] pub const BASE64_WRAP_WIDTH : usize = 64 ;
};
}
