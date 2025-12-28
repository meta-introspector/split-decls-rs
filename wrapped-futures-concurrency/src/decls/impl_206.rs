macro_rules! deps {
    () => {
        FutureGroup!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl < F : Future > FromIterator < F > for FutureGroup < F > { fn from_iter < T : IntoIterator < Item = F > > (iter : T) -> Self { let mut this = Self :: new () ; this . extend (iter) ; this } }
    };
}

impl_206!();