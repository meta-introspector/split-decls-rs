macro_rules! deps {
    () => {
        CowBytes!();
        SearcherRev!();
    };
}

macro_rules! FinderRev {
    () => {
        deps!();
        # [doc = " A single substring reverse searcher fixed to a particular needle."] # [doc = ""] # [doc = " The purpose of this type is to permit callers to construct a substring"] # [doc = " searcher that can be used to search haystacks without the overhead of"] # [doc = " constructing the searcher in the first place. This is a somewhat niche"] # [doc = " concern when it's necessary to re-use the same needle to search multiple"] # [doc = " different haystacks with as little overhead as possible. In general,"] # [doc = " using [`rfind`] is good enough, but `FinderRev` is useful when you can"] # [doc = " meaningfully observe searcher construction time in a profile."] # [doc = ""] # [doc = " When the `std` feature is enabled, then this type has an `into_owned`"] # [doc = " version which permits building a `FinderRev` that is not connected to"] # [doc = " the lifetime of its needle."] # [derive (Clone , Debug)] pub struct FinderRev < 'n > { needle : CowBytes < 'n > , searcher : SearcherRev , }
    };
}

FinderRev!()