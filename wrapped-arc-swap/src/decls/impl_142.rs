macro_rules! deps {
    () => {
        Guard!();
        RefCnt!();
        Strategy!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl < T : RefCnt , S : Strategy < T > > From < T > for Guard < T , S > { fn from (inner : T) -> Self { Self :: from_inner (inner) } }
    };
}

impl_142!()