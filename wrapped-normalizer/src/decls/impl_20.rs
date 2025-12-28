macro_rules! deps {
    () => {
        CanonicalCompositionBorrowed!();
        CanonicalComposition!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl CanonicalComposition { # [doc = " Constructs a borrowed version of this type for more efficient querying."] pub fn as_borrowed (& self) -> CanonicalCompositionBorrowed < '_ > { CanonicalCompositionBorrowed { canonical_compositions : self . canonical_compositions . get () , } } # [doc = " Constructs a new `CanonicalCompositionBorrowed` using compiled data."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [cfg (feature = "compiled_data")] # [expect (clippy :: new_ret_no_self)] pub const fn new () -> CanonicalCompositionBorrowed < 'static > { CanonicalCompositionBorrowed :: new () } icu_provider :: gen_buffer_data_constructors ! (() -> error : DataError , functions : [new : skip , try_new_with_buffer_provider , try_new_unstable , Self ,]) ; # [doc = icu_provider :: gen_buffer_unstable_docs ! (UNSTABLE , Self :: new)] pub fn try_new_unstable < D > (provider : & D) -> Result < Self , DataError > where D : DataProvider < NormalizerNfcV1 > + ? Sized , { let canonical_compositions : DataPayload < NormalizerNfcV1 > = provider . load (Default :: default ()) ? . payload ; Ok (CanonicalComposition { canonical_compositions , }) } }
    };
}

impl_20!()