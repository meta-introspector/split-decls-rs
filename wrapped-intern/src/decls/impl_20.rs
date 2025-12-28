macro_rules! deps {
    () => {
        InternStorage!();
        Internable!();
        InternMap!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < T : Internable + ? Sized > InternStorage < T > { fn get (& self) -> & InternMap < T > { self . map . get_or_init (| | DashMap :: < Arc < T > , () , BuildHasherDefault < FxHasher > > :: default ()) } }
    };
}

impl_20!()