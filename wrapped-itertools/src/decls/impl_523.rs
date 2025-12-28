macro_rules! deps {
    () => {
        CircularTupleWindows!();
        TupleCollect!();
    };
}

macro_rules! impl_523 {
    () => {
        deps!();
        impl < I , T > FusedIterator for CircularTupleWindows < I , T > where I : Iterator < Item = T :: Item > + Clone , T : TupleCollect + Clone , T :: Item : Clone , { }
    };
}

impl_523!()