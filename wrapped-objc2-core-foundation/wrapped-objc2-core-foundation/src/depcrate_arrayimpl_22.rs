// Generated macro for impl_22 (impl)
macro_rules! Depcrate_arrayimpl_22 {
() => {
// Module: crate::array
// Provides: {"impl_22"}
// Dependencies: {}
# [doc = " Convenience creation methods."] impl < T : ? Sized > CFArray < T > { # [doc = " Create a new empty `CFArray` capable of holding CoreFoundation"] # [doc = " objects."] # [inline] # [doc (alias = "CFArray::new")] pub fn empty () -> CFRetained < Self > where T : Type , { Self :: from_objects (& []) } # [doc = " Create a new `CFArray` with the given CoreFoundation objects."] # [inline] # [doc (alias = "CFArray::new")] pub fn from_objects (objects : & [& T]) -> CFRetained < Self > where T : Type , { let len = get_len (objects) ; let ptr = objects . as_ptr () . cast :: < * const c_void > () . cast_mut () ; let array = unsafe { CFArray :: new (None , ptr , len , & kCFTypeArrayCallBacks) } . unwrap_or_else (| | failed_creating_array (len)) ; unsafe { CFRetained :: cast_unchecked :: < Self > (array) } } # [doc = " Alias for easier transition from the `core-foundation` crate."] # [inline] # [allow (non_snake_case)] # [deprecated = "renamed to CFArray::from_objects"] pub fn from_CFTypes (objects : & [& T]) -> CFRetained < Self > where T : Type , { Self :: from_objects (objects) } # [doc = " Create a new `CFArray` with the given retained CoreFoundation objects."] # [inline] # [doc (alias = "CFArray::new")] pub fn from_retained_objects (objects : & [CFRetained < T >]) -> CFRetained < Self > where T : Type , { let len = get_len (objects) ; let ptr = objects . as_ptr () . cast :: < * const c_void > () . cast_mut () ; let array = unsafe { CFArray :: new (None , ptr , len , & kCFTypeArrayCallBacks) } . unwrap_or_else (| | failed_creating_array (len)) ; unsafe { CFRetained :: cast_unchecked :: < Self > (array) } } }
};
}
