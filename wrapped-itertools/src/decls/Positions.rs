macro_rules! Positions {
    () => {
        # [doc = " An iterator adapter to get the positions of each element that matches a predicate."] # [doc = ""] # [doc = " See [`.positions()`](crate::Itertools::positions) for more information."] # [derive (Clone)] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct Positions < I , F > { iter : Enumerate < I > , f : F , }
    };
}

Positions!()