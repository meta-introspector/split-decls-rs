// Generated macro for impl_23 (impl)
macro_rules! Depcrate_arrayimpl_23 {
() => {
// Module: crate::array
// Provides: {"impl_23"}
// Dependencies: {}
# [doc = " Convenience creation methods."] impl < T : ? Sized > CFMutableArray < T > { # [doc = " Create a new empty mutable array."] # [inline] # [doc (alias = "CFMutableArray::new")] pub fn empty () -> CFRetained < Self > where T : Type , { Self :: with_capacity (0) } # [doc = " Create a new mutable array with the given capacity."] # [inline] # [doc (alias = "CFMutableArray::new")] pub fn with_capacity (capacity : usize) -> CFRetained < Self > where T : Type , { let capacity = capacity . try_into () . expect ("capacity too high") ; let array = unsafe { CFMutableArray :: new (None , capacity , & kCFTypeArrayCallBacks) } . unwrap_or_else (| | failed_creating_array (capacity)) ; unsafe { CFRetained :: cast_unchecked :: < Self > (array) } } }
};
}
