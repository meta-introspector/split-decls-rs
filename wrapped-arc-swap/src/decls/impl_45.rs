macro_rules! deps {
    () => {
        Cache!();
        ArcSwapAny!();
        Access!();
        Strategy!();
        RefCnt!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < A , T , S > Access < T :: Target > for Cache < A , T > where A : Deref < Target = ArcSwapAny < T , S > > , T : Deref < Target = < T as RefCnt > :: Base > + RefCnt , S : Strategy < T > , { fn load (& mut self) -> & T :: Target { self . load () . deref () } }
    };
}

impl_45!();