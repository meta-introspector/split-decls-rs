// Generated macro for impl_106 (impl)
macro_rules! Depcrate_attributed_stringimpl_106 {
() => {
// Module: crate::attributed_string
// Provides: {"impl_106"}
// Dependencies: {}
impl NSAttributedString { # [doc = " Creates a new attributed string from the given string and attributes."] # [doc = ""] # [doc = " The attributes are associated with every UTF-16 code unit in the"] # [doc = " string."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The attributes must be valid."] # [doc (alias = "initWithString:")] # [cfg (feature = "NSDictionary")] # [cfg (feature = "NSString")] pub unsafe fn new_with_attributes (string : & NSString , attributes : & NSDictionary < NSAttributedStringKey , objc2 :: runtime :: AnyObject > ,) -> Retained < Self > { unsafe { Self :: initWithString_attributes (Self :: alloc () , string , Some (attributes)) } } # [doc = " Creates a new attributed string without any attributes."] # [doc (alias = "initWithString:")] # [cfg (feature = "NSString")] pub fn from_nsstring (string : & NSString) -> Retained < Self > { Self :: initWithString (Self :: alloc () , string) } }
};
}
