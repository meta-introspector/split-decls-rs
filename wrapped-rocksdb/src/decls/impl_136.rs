macro_rules! deps {
    () => {
        DBIteratorWithThreadMode!();
        DBAccess!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl < D : DBAccess > std :: iter :: FusedIterator for DBIteratorWithThreadMode < '_ , D > { }
    };
}

impl_136!()