macro_rules! deps {
    () => {
        UnescapeState!();
        Bytes!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl UnescapeState { # [doc = " Create a new `Bytes` variant with the given slice."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `bytes.len() > 11`."] fn bytes_raw (bytes : & [u8]) -> UnescapeState { assert ! (bytes . len () <= 11 , "no more than 11 bytes allowed") ; let mut buf = [0 ; 11] ; buf [.. bytes . len ()] . copy_from_slice (bytes) ; UnescapeState :: Bytes { buf , cur : 0 , len : bytes . len () } } # [doc = " Create a new `Bytes` variant with the prefix byte slice, followed by"] # [doc = " the UTF-8 encoding of the given char."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `prefix.len() > 3`."] fn bytes (prefix : & [u8] , ch : char) -> UnescapeState { assert ! (prefix . len () <= 3 , "no more than 3 bytes allowed") ; let mut buf = [0 ; 11] ; buf [.. prefix . len ()] . copy_from_slice (prefix) ; let chlen = ch . encode_utf8 (& mut buf [prefix . len () ..]) . len () ; UnescapeState :: Bytes { buf , cur : 0 , len : prefix . len () + chlen } } # [doc = " Create a new `Bytes` variant with the prefix byte slice, followed by"] # [doc = " the UTF-8 encoding of `ch1` and then `ch2`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `prefix.len() > 3`."] fn bytes2 (prefix : & [u8] , ch1 : char , ch2 : char) -> UnescapeState { assert ! (prefix . len () <= 3 , "no more than 3 bytes allowed") ; let mut buf = [0 ; 11] ; buf [.. prefix . len ()] . copy_from_slice (prefix) ; let len1 = ch1 . encode_utf8 (& mut buf [prefix . len () ..]) . len () ; let len2 = ch2 . encode_utf8 (& mut buf [prefix . len () + len1 ..]) . len () ; UnescapeState :: Bytes { buf , cur : 0 , len : prefix . len () + len1 + len2 } } }
    };
}

impl_54!()