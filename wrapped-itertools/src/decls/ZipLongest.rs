macro_rules! deps {
    () => {
        EitherOrBoth!();
    };
}

macro_rules! ZipLongest {
    () => {
        deps!();
        # [doc = " An iterator which iterates two other iterators simultaneously"] # [doc = " and wraps the elements in [`EitherOrBoth`]."] # [doc = ""] # [doc = " This iterator is *fused*."] # [doc = ""] # [doc = " See [`.zip_longest()`](crate::Itertools::zip_longest) for more information."] # [derive (Clone , Debug)] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct ZipLongest < T , U > { a : Fuse < T > , b : Fuse < U > , }
    };
}

ZipLongest!();