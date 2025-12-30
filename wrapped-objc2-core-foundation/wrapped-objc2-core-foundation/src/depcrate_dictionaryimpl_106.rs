// Generated macro for impl_106 (impl)
macro_rules! Depcrate_dictionaryimpl_106 {
() => {
// Module: crate::dictionary
// Provides: {"impl_106"}
// Dependencies: {}
# [doc = " Convenience creation methods."] impl < K : ? Sized , V : ? Sized > CFMutableDictionary < K , V > { # [doc = " Create a new empty mutable dictionary."] # [inline] # [doc (alias = "CFDictionaryCreateMutable")] pub fn empty () -> CFRetained < Self > where K : Type + PartialEq + Hash , V : Type , { Self :: with_capacity (0) } # [doc = " Create a new mutable dictionary with the given capacity."] # [inline] # [doc (alias = "CFDictionaryCreateMutable")] pub fn with_capacity (capacity : usize) -> CFRetained < Self > where K : Type + PartialEq + Hash , V : Type , { let capacity = capacity . try_into () . expect ("capacity too high") ; let dictionary = unsafe { CFMutableDictionary :: new (None , capacity , & kCFTypeDictionaryKeyCallBacks , & kCFTypeDictionaryValueCallBacks ,) } . unwrap_or_else (| | failed_creating_dictionary (capacity)) ; unsafe { CFRetained :: cast_unchecked :: < Self > (dictionary) } } }
};
}
