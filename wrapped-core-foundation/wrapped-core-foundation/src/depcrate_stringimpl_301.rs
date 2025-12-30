// Generated macro for impl_301 (impl)
macro_rules! Depcrate_stringimpl_301 {
() => {
// Module: crate::string
// Provides: {"impl_301"}
// Dependencies: {}
impl PartialEq < & str > for CFString { fn eq (& self , other : & & str) -> bool { unsafe { let temp = CFStringCreateWithBytesNoCopy (kCFAllocatorDefault , other . as_ptr () , other . len () . to_CFIndex () , kCFStringEncodingUTF8 , false as Boolean , kCFAllocatorNull ,) ; self . eq (& CFString :: wrap_under_create_rule (temp)) } } }
};
}
