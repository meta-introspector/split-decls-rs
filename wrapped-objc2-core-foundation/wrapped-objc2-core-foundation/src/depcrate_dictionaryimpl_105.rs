// Generated macro for impl_105 (impl)
macro_rules! Depcrate_dictionaryimpl_105 {
() => {
// Module: crate::dictionary
// Provides: {"impl_105"}
// Dependencies: {}
# [doc = " Convenience creation methods."] impl < K : ? Sized , V : ? Sized > CFDictionary < K , V > { # [doc = " Create a new empty dictionary."] # [inline] # [doc (alias = "CFDictionaryCreate")] pub fn empty () -> CFRetained < Self > where K : Type + PartialEq + Hash , V : Type , { Self :: from_slices (& [] , & []) } # [doc = " Create a new dictionary from slices of keys and values."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the slices have different lengths."] # [inline] # [doc (alias = "CFDictionaryCreate")] pub fn from_slices (keys : & [& K] , values : & [& V]) -> CFRetained < Self > where K : Type + PartialEq + Hash , V : Type , { assert_eq ! (keys . len () , values . len () , "key and object slices must have the same length" ,) ; debug_assert ! (keys . len () < CFIndex :: MAX as usize) ; let len = keys . len () as CFIndex ; let keys = keys . as_ptr () . cast :: < * const c_void > () . cast_mut () ; let values = values . as_ptr () . cast :: < * const c_void > () . cast_mut () ; let dictionary = unsafe { CFDictionary :: new (None , keys , values , len , & kCFTypeDictionaryKeyCallBacks , & kCFTypeDictionaryValueCallBacks ,) } . unwrap_or_else (| | failed_creating_dictionary (len)) ; unsafe { CFRetained :: cast_unchecked :: < Self > (dictionary) } } }
};
}
