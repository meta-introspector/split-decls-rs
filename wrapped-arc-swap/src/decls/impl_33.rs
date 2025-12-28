macro_rules! deps {
    () => {
        Guard!();
        RefCnt!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < 'a , T : RefCnt > Sealed for & 'a Guard < T > { }
    };
}

impl_33!();