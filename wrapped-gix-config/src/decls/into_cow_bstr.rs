macro_rules! into_cow_bstr {
    () => {
        pub (crate) fn into_cow_bstr (c : Cow < '_ , str >) -> Cow < '_ , BStr > { match c { Cow :: Borrowed (s) => Cow :: Borrowed (s . into ()) , Cow :: Owned (s) => Cow :: Owned (s . into ()) , } }
    };
}

into_cow_bstr!();