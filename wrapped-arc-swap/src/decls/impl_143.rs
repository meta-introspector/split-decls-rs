macro_rules! deps {
    () => {
        Guard!();
        RefCnt!();
        Strategy!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl < T : Default + RefCnt , S : Strategy < T > > Default for Guard < T , S > { fn default () -> Self { Self :: from (T :: default ()) } }
    };
}

impl_143!();