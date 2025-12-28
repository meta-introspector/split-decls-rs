macro_rules! deps {
    () => {
        HomogeneousTuple!();
    };
}

macro_rules! TupleWindows {
    () => {
        deps!();
        # [doc = " An iterator over all contiguous windows that produces tuples of a specific size."] # [doc = ""] # [doc = " See [`.tuple_windows()`](crate::Itertools::tuple_windows) for more"] # [doc = " information."] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone , Debug)] pub struct TupleWindows < I , T > where I : Iterator < Item = T :: Item > , T : HomogeneousTuple , { iter : I , last : Option < T > , }
    };
}

TupleWindows!()