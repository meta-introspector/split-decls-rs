macro_rules! deps {
    () => {
        Guard!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < T : Default + RefCnt , S : Strategy < T > > Default for Guard < T , S > { fn default () -> Self { Self :: from (T :: default ()) } }
    };
}

impl_15!()