macro_rules! deps {
    () => {
        ReadOnlyView!();
        DashMap!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl < K , V , S > ReadOnlyView < K , V , S > { pub (crate) fn new (map : DashMap < K , V , S >) -> Self { Self { map } } # [doc = " Consumes this `ReadOnlyView`, returning the underlying `DashMap`."] pub fn into_inner (self) -> DashMap < K , V , S > { self . map } }
    };
}

impl_87!()