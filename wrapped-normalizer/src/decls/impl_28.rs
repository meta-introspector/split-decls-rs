macro_rules! deps {
    () => {
        CanonicalDecomposition!();
        CanonicalDecompositionBorrowed!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl CanonicalDecomposition { # [doc = " Constructs a borrowed version of this type for more efficient querying."] pub fn as_borrowed (& self) -> CanonicalDecompositionBorrowed < '_ > { CanonicalDecompositionBorrowed { decompositions : self . decompositions . get () , tables : self . tables . get () , non_recursive : self . non_recursive . get () , } } # [doc = " Construct from compiled data."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [cfg (feature = "compiled_data")] # [expect (clippy :: new_ret_no_self)] pub const fn new () -> CanonicalDecompositionBorrowed < 'static > { CanonicalDecompositionBorrowed :: new () } icu_provider :: gen_buffer_data_constructors ! (() -> error : DataError , functions : [new : skip , try_new_with_buffer_provider , try_new_unstable , Self ,]) ; # [doc = icu_provider :: gen_buffer_unstable_docs ! (UNSTABLE , Self :: new)] pub fn try_new_unstable < D > (provider : & D) -> Result < Self , DataError > where D : DataProvider < NormalizerNfdDataV1 > + DataProvider < NormalizerNfdTablesV1 > + DataProvider < NormalizerNfdSupplementV1 > + ? Sized , { let decompositions : DataPayload < NormalizerNfdDataV1 > = provider . load (Default :: default ()) ? . payload ; let tables : DataPayload < NormalizerNfdTablesV1 > = provider . load (Default :: default ()) ? . payload ; if tables . get () . scalars16 . len () + tables . get () . scalars24 . len () > 0xFFF { return Err (DataError :: custom ("future extension")) ; } let non_recursive : DataPayload < NormalizerNfdSupplementV1 > = provider . load (Default :: default ()) ? . payload ; Ok (CanonicalDecomposition { decompositions , tables , non_recursive , }) } }
    };
}

impl_28!();