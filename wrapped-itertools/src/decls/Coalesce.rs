macro_rules! deps {
    () => {
        CoalesceBy!();
        NoCount!();
    };
}

macro_rules! Coalesce {
    () => {
        deps!();
        # [doc = " An iterator adaptor that may join together adjacent elements."] # [doc = ""] # [doc = " See [`.coalesce()`](crate::Itertools::coalesce) for more information."] pub type Coalesce < I , F > = CoalesceBy < I , F , NoCount > ;
    };
}

Coalesce!();