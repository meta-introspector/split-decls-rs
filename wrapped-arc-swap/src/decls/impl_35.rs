macro_rules! deps {
    () => {
        Guard!();
        RefCnt!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < T : RefCnt > Sealed for Guard < T > { }
    };
}

impl_35!();