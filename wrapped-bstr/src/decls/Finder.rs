macro_rules! Finder {
    () => {
        # [doc = " A single substring searcher fixed to a particular needle."] # [doc = ""] # [doc = " The purpose of this type is to permit callers to construct a substring"] # [doc = " searcher that can be used to search haystacks without the overhead of"] # [doc = " constructing the searcher in the first place. This is a somewhat niche"] # [doc = " concern when it's necessary to re-use the same needle to search multiple"] # [doc = " different haystacks with as little overhead as possible. In general, using"] # [doc = " [`ByteSlice::find`](trait.ByteSlice.html#method.find)"] # [doc = " or"] # [doc = " [`ByteSlice::find_iter`](trait.ByteSlice.html#method.find_iter)"] # [doc = " is good enough, but `Finder` is useful when you can meaningfully observe"] # [doc = " searcher construction time in a profile."] # [doc = ""] # [doc = " When the `std` feature is enabled, then this type has an `into_owned`"] # [doc = " version which permits building a `Finder` that is not connected to the"] # [doc = " lifetime of its needle."] # [derive (Clone , Debug)] pub struct Finder < 'a > (memmem :: Finder < 'a >) ;
    };
}

Finder!();