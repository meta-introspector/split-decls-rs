macro_rules! impl_const_get_with_index_for_integer {
    () => {
        macro_rules ! impl_const_get_with_index_for_integer { ($ integer : ty) => { impl <'a , V > LiteMap <$ integer , V , &'a [($ integer , V)] > { # [doc = " Const function to get the value associated with an integer key, if it exists."] # [doc = ""] # [doc = " Note: This function will no longer be needed if const trait behavior is stabilized."] # [doc = ""] # [doc = " Also returns the index of the value."] pub const fn const_get_with_index (& self , key : $ integer) -> Option < (usize , &'a V) > { let mut i = 0 ; let mut j = self . const_len () ; while i < j { let mid = (i + j) / 2 ; # [expect (clippy :: indexing_slicing)] let x = & self . values [mid] ; if key == x . 0 { return Some ((mid , & x . 1)) ; } else if key > x . 0 { i = mid + 1 ; } else { j = mid ; } } return None ; } } } ; }
    };
}

impl_const_get_with_index_for_integer!();