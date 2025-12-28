macro_rules! normalize_bstring {
    () => {
        # [doc = " `Vec[u8]` variant of [`normalize`]."] # [must_use] pub fn normalize_bstring (input : impl Into < BString >) -> Cow < 'static , BStr > { normalize (Cow :: Owned (input . into ())) }
    };
}

normalize_bstring!();