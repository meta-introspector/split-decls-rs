macro_rules! deps {
    () => {
        Replacer!();
    };
}

macro_rules! ReplacerRef {
    () => {
        deps!();
        # [doc = " A by-reference adaptor for a [`Replacer`]."] # [doc = ""] # [doc = " This permits reusing the same `Replacer` value in multiple calls to a"] # [doc = " replacement routine like [`Regex::replace_all`]."] # [doc = ""] # [doc = " This type is created by [`Replacer::by_ref`]."] # [derive (Debug)] pub struct ReplacerRef < 'a , R : ? Sized > (& 'a mut R) ;
    };
}

ReplacerRef!();