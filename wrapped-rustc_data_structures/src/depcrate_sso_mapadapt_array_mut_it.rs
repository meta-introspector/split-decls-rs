// Generated macro for adapt_array_mut_it (function)
macro_rules! Depcrate_sso_mapadapt_array_mut_it {
() => {
// Module: crate::sso::map
// Provides: {"adapt_array_mut_it"}
// Dependencies: {}
# [doc = " adapts Item of array mut reference iterator to Item of hashmap mut reference iterator."] # [inline (always)] fn adapt_array_mut_it < K , V > (pair : & mut (K , V)) -> (& K , & mut V) { let (a , b) = pair ; (a , b) }
};
}
