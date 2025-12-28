macro_rules! ZipEq {
    () => {
        # [doc = " An iterator which iterates two other iterators simultaneously"] # [doc = " and panic if they have different lengths."] # [doc = ""] # [doc = " See [`.zip_eq()`](crate::Itertools::zip_eq) for more information."] # [derive (Clone , Debug)] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct ZipEq < I , J > { a : I , b : J , }
    };
}

ZipEq!();