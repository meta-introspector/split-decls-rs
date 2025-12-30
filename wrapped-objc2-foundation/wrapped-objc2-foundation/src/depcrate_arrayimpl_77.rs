// Generated macro for impl_77 (impl)
macro_rules! Depcrate_arrayimpl_77 {
() => {
// Module: crate::array
// Provides: {"impl_77"}
// Dependencies: {}
# [doc = " Convenience creation methods."] impl < ObjectType : Message > NSArray < ObjectType > { # [doc = " Create a new array from a slice of objects."] # [doc = ""] # [doc = " This is a safe interface to `initWithObjects:count:`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use objc2_foundation::{NSArray, ns_string};"] # [doc = ""] # [doc = " let array = NSArray::from_slice(&["] # [doc = "     ns_string!(\"abc\"),"] # [doc = "     ns_string!(\"def\"),"] # [doc = "     ns_string!(\"ghi\"),"] # [doc = " ]);"] # [doc = " ```"] # [doc (alias = "initWithObjects:count:")] pub fn from_slice (slice : & [& ObjectType]) -> Retained < Self > { let len = slice . len () ; let ptr = util :: ref_ptr_cast_const (slice . as_ptr ()) ; unsafe { Self :: initWithObjects_count (Self :: alloc () , ptr , len) } } # [doc = " Create a new array from a slice of retained objects."] # [doc = ""] # [doc = " This is a safe interface to `initWithObjects:count:`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use objc2_foundation::{NSArray, NSObject};"] # [doc = ""] # [doc = " let array = NSArray::from_retained_slice(&["] # [doc = "     NSObject::new(),"] # [doc = "     NSObject::new(),"] # [doc = "     NSObject::new(),"] # [doc = " ]);"] # [doc = " ```"] # [doc (alias = "initWithObjects:count:")] pub fn from_retained_slice (slice : & [Retained < ObjectType >]) -> Retained < Self > { let len = slice . len () ; let ptr = util :: retained_ptr_cast_const (slice . as_ptr ()) ; unsafe { Self :: initWithObjects_count (Self :: alloc () , ptr , len) } } }
};
}
