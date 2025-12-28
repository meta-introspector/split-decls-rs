macro_rules! deps {
    () => {
        DBAccess!();
        DBRawIteratorWithThreadMode!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        unsafe impl < D : DBAccess > Send for DBRawIteratorWithThreadMode < '_ , D > { }
    };
}

impl_127!();