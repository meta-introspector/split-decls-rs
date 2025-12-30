// Generated macro for impl_2325 (impl)
macro_rules! Depcrate_setimpl_2325 {
() => {
// Module: crate::set
// Provides: {"impl_2325"}
// Dependencies: {}
# [doc = " Convenience creation methods."] impl < ObjectType : Message > NSMutableSet < ObjectType > { # [doc = " Creates an [`NSMutableSet`] from a slice of `Retained`s."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use objc2_foundation::{NSMutableSet, NSString};"] # [doc = ""] # [doc = " let strs = [\"one\", \"two\", \"three\"].map(NSString::from_str);"] # [doc = " let set = NSMutableSet::from_retained_slice(&strs);"] # [doc = " ```"] pub fn from_retained_slice (slice : & [Retained < ObjectType >]) -> Retained < Self > { let len = slice . len () ; let ptr = util :: retained_ptr_cast_const (slice . as_ptr ()) ; unsafe { Self :: initWithObjects_count (Self :: alloc () , ptr , len) } } pub fn from_slice (slice : & [& ObjectType]) -> Retained < Self > { let len = slice . len () ; let ptr = util :: ref_ptr_cast_const (slice . as_ptr ()) ; unsafe { Self :: initWithObjects_count (Self :: alloc () , ptr , len) } } }
};
}
