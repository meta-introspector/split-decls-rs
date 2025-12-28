macro_rules! deps {
    () => {
        DBAccess!();
        DBRawIteratorWithThreadMode!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        unsafe impl < D : DBAccess > Sync for DBRawIteratorWithThreadMode < '_ , D > { }
    };
}

impl_128!()