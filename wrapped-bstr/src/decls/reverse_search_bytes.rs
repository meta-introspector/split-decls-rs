macro_rules! reverse_search_bytes {
    () => {
        # [doc = " Safe wrapper around `reverse_search`"] # [inline] pub (crate) fn reverse_search_bytes < F : Fn (u8) -> bool > (s : & [u8] , confirm : F ,) -> Option < usize > { unsafe { let start = s . as_ptr () ; let end = start . add (s . len ()) ; reverse_search (start , end , end , confirm) } }
    };
}

reverse_search_bytes!()