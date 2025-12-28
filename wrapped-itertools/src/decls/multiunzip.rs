macro_rules! deps {
    () => {
        MultiUnzip!();
    };
}

macro_rules! multiunzip {
    () => {
        deps!();
        # [doc = " Converts an iterator of tuples into a tuple of containers."] # [doc = ""] # [doc = " `multiunzip()` consumes an entire iterator of n-ary tuples, producing `n` collections, one for each"] # [doc = " column."] # [doc = ""] # [doc = " This function is, in some sense, the opposite of [`multizip`]."] # [doc = ""] # [doc = " ```"] # [doc = " use itertools::multiunzip;"] # [doc = ""] # [doc = " let inputs = vec![(1, 2, 3), (4, 5, 6), (7, 8, 9)];"] # [doc = ""] # [doc = " let (a, b, c): (Vec<_>, Vec<_>, Vec<_>) = multiunzip(inputs);"] # [doc = ""] # [doc = " assert_eq!(a, vec![1, 4, 7]);"] # [doc = " assert_eq!(b, vec![2, 5, 8]);"] # [doc = " assert_eq!(c, vec![3, 6, 9]);"] # [doc = " ```"] # [doc = ""] # [doc = " [`multizip`]: crate::multizip"] pub fn multiunzip < FromI , I > (i : I) -> FromI where I : IntoIterator , I :: IntoIter : MultiUnzip < FromI > , { i . into_iter () . multiunzip () }
    };
}

multiunzip!()