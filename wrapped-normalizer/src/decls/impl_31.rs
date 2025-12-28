macro_rules! deps {
    () => {
        CanonicalCombiningClassMap!();
        CanonicalCombiningClassMapBorrowed!();
        Baked!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl CanonicalCombiningClassMapBorrowed < 'static > { # [doc = " Cheaply converts a [`CanonicalCombiningClassMapBorrowed<'static>`] into a [`CanonicalCombiningClassMap`]."] # [doc = ""] # [doc = " Note: Due to branching and indirection, using [`CanonicalCombiningClassMap`] might inhibit some"] # [doc = " compile-time optimizations that are possible with [`CanonicalCombiningClassMapBorrowed`]."] pub const fn static_to_owned (self) -> CanonicalCombiningClassMap { CanonicalCombiningClassMap { decompositions : DataPayload :: from_static_ref (self . decompositions) , } } # [doc = " Construct from compiled data."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [cfg (feature = "compiled_data")] pub const fn new () -> Self { CanonicalCombiningClassMapBorrowed { decompositions : crate :: provider :: Baked :: SINGLETON_NORMALIZER_NFD_DATA_V1 , } } }
    };
}

impl_31!()