// Generated macro for impl_78 (impl)
macro_rules! Depcrate_arrayimpl_78 {
() => {
// Module: crate::array
// Provides: {"impl_78"}
// Dependencies: {}
# [doc = " Convenience creation methods."] impl < ObjectType : Message > NSMutableArray < ObjectType > { # [doc (alias = "initWithObjects:count:")] pub fn from_slice (slice : & [& ObjectType]) -> Retained < Self > { let len = slice . len () ; let ptr = util :: ref_ptr_cast_const (slice . as_ptr ()) ; unsafe { Self :: initWithObjects_count (Self :: alloc () , ptr , len) } } # [doc (alias = "initWithObjects:count:")] pub fn from_retained_slice (slice : & [Retained < ObjectType >]) -> Retained < Self > { let len = slice . len () ; let ptr = util :: retained_ptr_cast_const (slice . as_ptr ()) ; unsafe { Self :: initWithObjects_count (Self :: alloc () , ptr , len) } } }
};
}
