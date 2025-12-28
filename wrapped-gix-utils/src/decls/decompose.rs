macro_rules! decompose {
    () => {
        # [doc = " Assure that `s` is decomposed, i.e. `ä` turns into `a` and `<umlaut>`."] # [doc = ""] # [doc = " At the expense of extra-compute, it does nothing if there is no work to be done, returning the original input without allocating."] pub fn decompose (s : Cow < '_ , str >) -> Cow < '_ , str > { use unicode_normalization :: { is_nfd , UnicodeNormalization } ; if is_nfd (s . as_ref ()) { s } else { Cow :: Owned (s . as_ref () . nfd () . collect ()) } }
    };
}

decompose!();