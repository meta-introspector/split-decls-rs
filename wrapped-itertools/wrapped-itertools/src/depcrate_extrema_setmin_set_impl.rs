// Generated macro for min_set_impl (function)
macro_rules! Depcrate_extrema_setmin_set_impl {
() => {
// Module: crate::extrema_set
// Provides: {"min_set_impl"}
// Dependencies: {}
# [doc = " Implementation guts for `min_set`, `min_set_by`, and `min_set_by_key`."] pub fn min_set_impl < I , K , F , Compare > (mut it : I , mut key_for : F , mut compare : Compare ,) -> Vec < I :: Item > where I : Iterator , F : FnMut (& I :: Item) -> K , Compare : FnMut (& I :: Item , & I :: Item , & K , & K) -> Ordering , { match it . next () { None => Vec :: new () , Some (element) => { let mut current_key = key_for (& element) ; let mut result = vec ! [element] ; it . for_each (| element | { let key = key_for (& element) ; match compare (& element , & result [0] , & key , & current_key) { Ordering :: Less => { result . clear () ; result . push (element) ; current_key = key ; } Ordering :: Equal => { result . push (element) ; } Ordering :: Greater => { } } }) ; result } } }
};
}
