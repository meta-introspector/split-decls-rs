macro_rules! add_prefix {
    () => {
        # [cfg (any (feature = "tar" , feature = "tar_gz"))] fn add_prefix < 'a > (relative_path : & 'a bstr :: BStr , prefix : Option < & bstr :: BString >) -> std :: borrow :: Cow < 'a , bstr :: BStr > { use std :: borrow :: Cow ; match prefix { None => Cow :: Borrowed (relative_path) , Some (prefix) => { use bstr :: ByteVec ; let mut buf = prefix . clone () ; buf . push_str (relative_path) ; Cow :: Owned (buf) } } }
    };
}

add_prefix!();