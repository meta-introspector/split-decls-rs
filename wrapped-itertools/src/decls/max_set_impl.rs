macro_rules! max_set_impl {
    () => {
        # [doc = " Implementation guts for `ax_set`, `max_set_by`, and `max_set_by_key`."] pub fn max_set_impl < I , K , F , Compare > (it : I , key_for : F , mut compare : Compare) -> Vec < I :: Item > where I : Iterator , F : FnMut (& I :: Item) -> K , Compare : FnMut (& I :: Item , & I :: Item , & K , & K) -> Ordering , { min_set_impl (it , key_for , | it1 , it2 , key1 , key2 | { compare (it2 , it1 , key2 , key1) }) }
    };
}

max_set_impl!()