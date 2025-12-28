macro_rules! deps {
    () => {
        Split!();
        OsStrExt!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl OsStrExt for OsStr { fn try_str (& self) -> Result < & str , std :: str :: Utf8Error > { let bytes = self . as_encoded_bytes () ; std :: str :: from_utf8 (bytes) } fn contains (& self , needle : & str) -> bool { self . find (needle) . is_some () } fn find (& self , needle : & str) -> Option < usize > { let bytes = self . as_encoded_bytes () ; (0 ..= self . len () . checked_sub (needle . len ()) ?) . find (| & x | bytes [x ..] . starts_with (needle . as_bytes ())) } fn strip_prefix (& self , prefix : & str) -> Option < & OsStr > { let bytes = self . as_encoded_bytes () ; bytes . strip_prefix (prefix . as_bytes ()) . map (| s | { unsafe { OsStr :: from_encoded_bytes_unchecked (s) } }) } fn starts_with (& self , prefix : & str) -> bool { let bytes = self . as_encoded_bytes () ; bytes . starts_with (prefix . as_bytes ()) } fn split < 's , 'n > (& 's self , needle : & 'n str) -> Split < 's , 'n > { assert_ne ! (needle , "") ; Split { haystack : Some (self) , needle , } } fn split_once (& self , needle : & '_ str) -> Option < (& OsStr , & OsStr) > { let start = self . find (needle) ? ; let end = start + needle . len () ; let haystack = self . as_encoded_bytes () ; let first = & haystack [0 .. start] ; let second = & haystack [end ..] ; unsafe { Some ((OsStr :: from_encoded_bytes_unchecked (first) , OsStr :: from_encoded_bytes_unchecked (second) ,)) } } }
    };
}

impl_1!();