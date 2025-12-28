macro_rules! deps {
    () => {
        RefCnt!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < 'a , T : RefCnt > Sealed for & 'a T { }
    };
}

impl_31!();