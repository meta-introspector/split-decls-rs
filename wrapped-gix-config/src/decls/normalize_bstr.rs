macro_rules! normalize_bstr {
    () => {
        # [doc = " `&[u8]` variant of [`normalize`]."] # [must_use] pub fn normalize_bstr < 'a > (input : impl Into < & 'a BStr >) -> Cow < 'a , BStr > { normalize (Cow :: Borrowed (input . into ())) }
    };
}

normalize_bstr!()