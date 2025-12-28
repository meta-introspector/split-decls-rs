macro_rules! deps {
    () => {
        DBAccess!();
        DBIteratorWithThreadMode!();
        DBRawIteratorWithThreadMode!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl < 'a , D : DBAccess > Into < DBRawIteratorWithThreadMode < 'a , D > > for DBIteratorWithThreadMode < 'a , D > { fn into (self) -> DBRawIteratorWithThreadMode < 'a , D > { self . raw } }
    };
}

impl_137!()