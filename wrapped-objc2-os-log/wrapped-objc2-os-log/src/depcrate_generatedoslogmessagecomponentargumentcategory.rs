// Generated macro for OSLogMessageComponentArgumentCategory (struct)
macro_rules! Depcrate_generatedOSLogMessageComponentArgumentCategory {
() => {
// Module: crate::generated
// Provides: {"OSLogMessageComponentArgumentCategory"}
// Dependencies: {}
# [doc = " The kind of data corresponding to an argument in a message"] # [doc = " payload, like the number associated with a \"%d\" placeholder."] # [doc = " This value can be undefined if the argument data cannot be"] # [doc = " decoded; for example, it may be redacted."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/oslog/oslogmessagecomponentargumentcategory?language=objc)"] # [repr (transparent)] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash , PartialOrd , Ord , Default)] pub struct OSLogMessageComponentArgumentCategory (pub NSInteger) ;
};
}
