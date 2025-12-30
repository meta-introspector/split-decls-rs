// Generated macro for Variant (enum)
macro_rules! DepcrateVariant {
() => {
// Module: crate
// Provides: {"Variant"}
// Dependencies: {}
# [doc = " Which variant of the encoding are we working with?"] # [derive (PartialEq , Eq)] enum Variant { # [doc = " Regular CESU-8, with '\\0' represented by itself."] Standard , # [doc = " This is technically Java's \"Modified UTF-8\", which is supposedly"] # [doc = " like CESU-8, except that it UTF-8 encodes the '\\0' byte.  I'm sure"] # [doc = " it seemed like a good idea at the time."] Java , }
};
}
