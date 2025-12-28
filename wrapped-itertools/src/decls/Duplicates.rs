macro_rules! deps {
    () => {
        DuplicatesBy!();
    };
}

macro_rules! Duplicates {
    () => {
        deps!();
        # [doc = " An iterator adapter to filter out duplicate elements."] # [doc = ""] # [doc = " See [`.duplicates()`](crate::Itertools::duplicates) for more information."] pub type Duplicates < I > = private :: DuplicatesBy < I , < I as Iterator > :: Item , private :: ById > ;
    };
}

Duplicates!()