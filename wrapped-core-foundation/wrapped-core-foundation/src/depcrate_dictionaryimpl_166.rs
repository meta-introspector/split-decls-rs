// Generated macro for impl_166 (impl)
macro_rules! Depcrate_dictionaryimpl_166 {
() => {
// Module: crate::dictionary
// Provides: {"impl_166"}
// Dependencies: {}
impl < 'a , K , V > From < & 'a CFDictionary < K , V > > for CFMutableDictionary < K , V > { # [doc = " Creates a new mutable dictionary with the key-value pairs from another dictionary."] # [doc = " The capacity of the new mutable dictionary is not limited."] fn from (dict : & 'a CFDictionary < K , V >) -> Self { unsafe { let mut_dict_ref = CFDictionaryCreateMutableCopy (kCFAllocatorDefault , 0 , dict . 0) ; TCFType :: wrap_under_create_rule (mut_dict_ref) } } }
};
}
