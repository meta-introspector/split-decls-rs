// Generated macro for impl_26 (impl)
macro_rules! Depcrate_arrayimpl_26 {
() => {
// Module: crate::array
// Provides: {"impl_26"}
// Dependencies: {}
# [doc = " Convenience mutation methods."] impl < T > CFMutableArray < T > { # [doc = " Push an object to the end of the array."] # [inline] # [doc (alias = "CFArrayAppendValue")] pub fn append (& self , obj : & T) { let ptr : * const T = obj ; let ptr : * const c_void = ptr . cast () ; unsafe { CFMutableArray :: append_value (Some (self . as_opaque ()) , ptr) } } # [doc = " Insert an object into the array at the given index."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the index is out of bounds."] # [doc (alias = "CFArrayInsertValueAtIndex")] pub fn insert (& self , index : usize , obj : & T) { let len = self . len () ; if index <= len { let ptr : * const T = obj ; let ptr : * const c_void = ptr . cast () ; unsafe { CFMutableArray :: insert_value_at_index (Some (self . as_opaque ()) , index as CFIndex , ptr) } } else { panic ! ("insertion index (is {}) should be <= len (is {})" , index , len) ; } } }
};
}
