// Generated macro for impl_107 (impl)
macro_rules! Depcrate_attributed_stringimpl_107 {
() => {
// Module: crate::attributed_string
// Provides: {"impl_107"}
// Dependencies: {}
impl NSMutableAttributedString { # [doc (alias = "initWithString:")] # [cfg (feature = "NSString")] pub fn from_nsstring (string : & NSString) -> Retained < Self > { Self :: initWithString (Self :: alloc () , string) } # [doc (alias = "initWithAttributedString:")] pub fn from_attributed_nsstring (attributed_string : & NSAttributedString) -> Retained < Self > { Self :: initWithAttributedString (Self :: alloc () , attributed_string) } }
};
}
