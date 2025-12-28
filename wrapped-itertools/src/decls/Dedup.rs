macro_rules! deps {
    () => {
        DedupBy!();
        DedupEq!();
    };
}

macro_rules! Dedup {
    () => {
        deps!();
        # [doc = " An iterator adaptor that removes repeated duplicates."] # [doc = ""] # [doc = " See [`.dedup()`](crate::Itertools::dedup) for more information."] pub type Dedup < I > = DedupBy < I , DedupEq > ;
    };
}

Dedup!()