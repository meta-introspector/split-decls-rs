macro_rules! deps {
    () => {
        TupleCollect!();
        TupleWindows!();
    };
}

macro_rules! CircularTupleWindows {
    () => {
        deps!();
        # [doc = " An iterator over all windows, wrapping back to the first elements when the"] # [doc = " window would otherwise exceed the length of the iterator, producing tuples"] # [doc = " of a specific size."] # [doc = ""] # [doc = " See [`.circular_tuple_windows()`](crate::Itertools::circular_tuple_windows) for more"] # [doc = " information."] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Debug , Clone)] pub struct CircularTupleWindows < I , T > where I : Iterator < Item = T :: Item > + Clone , T : TupleCollect + Clone , { iter : TupleWindows < Cycle < I > , T > , len : usize , }
    };
}

CircularTupleWindows!()