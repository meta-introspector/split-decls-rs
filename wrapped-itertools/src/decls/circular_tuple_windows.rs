macro_rules! deps {
    () => {
        TupleCollect!();
        CircularTupleWindows!();
    };
}

macro_rules! circular_tuple_windows {
    () => {
        deps!();
        pub fn circular_tuple_windows < I , T > (iter : I) -> CircularTupleWindows < I , T > where I : Iterator < Item = T :: Item > + Clone + ExactSizeIterator , T : TupleCollect + Clone , T :: Item : Clone , { let len = iter . len () ; let iter = tuple_windows (iter . cycle ()) ; CircularTupleWindows { iter , len } }
    };
}

circular_tuple_windows!();