macro_rules! deps {
    () => {
        CanonicalDecompositionBorrowed!();
        CanonicalDecomposition!();
        Baked!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl CanonicalDecompositionBorrowed < 'static > { # [doc = " Cheaply converts a [`CanonicalDecompositionBorrowed<'static>`] into a [`CanonicalDecomposition`]."] # [doc = ""] # [doc = " Note: Due to branching and indirection, using [`CanonicalDecomposition`] might inhibit some"] # [doc = " compile-time optimizations that are possible with [`CanonicalDecompositionBorrowed`]."] pub const fn static_to_owned (self) -> CanonicalDecomposition { CanonicalDecomposition { decompositions : DataPayload :: from_static_ref (self . decompositions) , tables : DataPayload :: from_static_ref (self . tables) , non_recursive : DataPayload :: from_static_ref (self . non_recursive) , } } # [doc = " Construct from compiled data."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [cfg (feature = "compiled_data")] pub const fn new () -> Self { const _ : () = assert ! (crate :: provider :: Baked :: SINGLETON_NORMALIZER_NFD_TABLES_V1 . scalars16 . const_len () + crate :: provider :: Baked :: SINGLETON_NORMALIZER_NFD_TABLES_V1 . scalars24 . const_len () <= 0xFFF , "future extension") ; Self { decompositions : crate :: provider :: Baked :: SINGLETON_NORMALIZER_NFD_DATA_V1 , tables : crate :: provider :: Baked :: SINGLETON_NORMALIZER_NFD_TABLES_V1 , non_recursive : crate :: provider :: Baked :: SINGLETON_NORMALIZER_NFD_SUPPLEMENT_V1 , } } }
    };
}

impl_24!()