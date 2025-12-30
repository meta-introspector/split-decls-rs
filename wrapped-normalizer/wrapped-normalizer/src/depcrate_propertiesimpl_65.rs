// Generated macro for impl_65 (impl)
macro_rules! Depcrate_propertiesimpl_65 {
() => {
// Module: crate::properties
// Provides: {"impl_65"}
// Dependencies: {}
impl CanonicalCombiningClassMap { # [doc = " Constructs a borrowed version of this type for more efficient querying."] pub fn as_borrowed (& self) -> CanonicalCombiningClassMapBorrowed < '_ > { CanonicalCombiningClassMapBorrowed { decompositions : self . decompositions . get () , } } # [doc = " Construct from compiled data."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [cfg (feature = "compiled_data")] # [expect (clippy :: new_ret_no_self)] pub const fn new () -> CanonicalCombiningClassMapBorrowed < 'static > { CanonicalCombiningClassMapBorrowed :: new () } icu_provider :: gen_buffer_data_constructors ! (() -> error : DataError , functions : [new : skip , try_new_with_buffer_provider , try_new_unstable , Self ,]) ; # [doc = icu_provider :: gen_buffer_unstable_docs ! (UNSTABLE , Self :: new)] pub fn try_new_unstable < D > (provider : & D) -> Result < Self , DataError > where D : DataProvider < NormalizerNfdDataV1 > + ? Sized , { let decompositions : DataPayload < NormalizerNfdDataV1 > = provider . load (Default :: default ()) ? . payload ; Ok (CanonicalCombiningClassMap { decompositions }) } }
};
}
