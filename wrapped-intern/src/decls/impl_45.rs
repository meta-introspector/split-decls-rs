macro_rules! deps {
    () => {
        InternMap!();
        Internable!();
        InternStorage!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < T : Internable + ? Sized > InternStorage < T > { fn get (& self) -> & InternMap < T > { self . map . get_or_init (| | DashMap :: < Arc < T > , () , BuildHasherDefault < FxHasher > > :: default ()) } }
    };
}

impl_45!();