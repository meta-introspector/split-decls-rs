// Generated macro for adapt_array_ref_it (function)
macro_rules! Depcrate_sso_mapadapt_array_ref_it {
() => {
// Module: crate::sso::map
// Provides: {"adapt_array_ref_it"}
// Dependencies: {}
# [doc = " adapts Item of array reference iterator to Item of hashmap reference iterator."] # [inline (always)] fn adapt_array_ref_it < K , V > (pair : & (K , V)) -> (& K , & V) { let (a , b) = pair ; (a , b) }
};
}
