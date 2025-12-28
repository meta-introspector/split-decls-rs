macro_rules! precompose {
    () => {
        # [doc = " Assure that `s` is precomposed, i.e. `ä` is a single code-point, and not two i.e. `a` and `<umlaut>`."] # [doc = ""] # [doc = " At the expense of extra-compute, it does nothing if there is no work to be done, returning the original input without allocating."] pub fn precompose (s : Cow < '_ , str >) -> Cow < '_ , str > { use unicode_normalization :: { is_nfc , UnicodeNormalization } ; if is_nfc (s . as_ref ()) { s } else { Cow :: Owned (s . as_ref () . nfc () . collect ()) } }
    };
}

precompose!()