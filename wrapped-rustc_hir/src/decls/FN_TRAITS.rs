macro_rules! FN_TRAITS {
    () => {
        pub static FN_TRAITS : & 'static [LangItem] = & [LangItem :: Fn , LangItem :: FnMut , LangItem :: FnOnce] ;
    };
}

FN_TRAITS!();