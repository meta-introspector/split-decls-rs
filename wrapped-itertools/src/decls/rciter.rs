macro_rules! deps {
    () => {
        RcIter!();
    };
}

macro_rules! rciter {
    () => {
        deps!();
        # [doc = " Return an iterator inside a `Rc<RefCell<_>>` wrapper."] # [doc = ""] # [doc = " The returned `RcIter` can be cloned, and each clone will refer back to the"] # [doc = " same original iterator."] # [doc = ""] # [doc = " `RcIter` allows doing interesting things like using `.zip()` on an iterator with"] # [doc = " itself, at the cost of runtime borrow checking which may have a performance"] # [doc = " penalty."] # [doc = ""] # [doc = " Iterator element type is `Self::Item`."] # [doc = ""] # [doc = " ```"] # [doc = " use itertools::rciter;"] # [doc = " use itertools::zip;"] # [doc = ""] # [doc = " // In this example a range iterator is created and we iterate it using"] # [doc = " // three separate handles (two of them given to zip)."] # [doc = " // We also use the IntoIterator implementation for `&RcIter`."] # [doc = ""] # [doc = " let mut iter = rciter(0..9);"] # [doc = " let mut z = zip(&iter, &iter);"] # [doc = ""] # [doc = " assert_eq!(z.next(), Some((0, 1)));"] # [doc = " assert_eq!(z.next(), Some((2, 3)));"] # [doc = " assert_eq!(z.next(), Some((4, 5)));"] # [doc = " assert_eq!(iter.next(), Some(6));"] # [doc = " assert_eq!(z.next(), Some((7, 8)));"] # [doc = " assert_eq!(z.next(), None);"] # [doc = " ```"] # [doc = ""] # [doc = " **Panics** in iterator methods if a borrow error is encountered in the"] # [doc = " iterator methods. It can only happen if the `RcIter` is reentered in"] # [doc = " `.next()`, i.e. if it somehow participates in an “iterator knot”"] # [doc = " where it is an adaptor of itself."] pub fn rciter < I > (iterable : I) -> RcIter < I :: IntoIter > where I : IntoIterator , { RcIter { rciter : Rc :: new (RefCell :: new (iterable . into_iter ())) , } }
    };
}

rciter!();