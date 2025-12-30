// Generated macro for impl_29 (impl)
macro_rules! Depcrate_closerimpl_29 {
() => {
// Module: crate::closer
// Provides: {"impl_29"}
// Dependencies: {}
impl < CM : AsRef < CaseMapper > > CaseMapCloser < CM > { icu_provider :: gen_buffer_data_constructors ! ((casemapper : CM) -> error : DataError , functions : [new_with_mapper : skip , try_new_with_mapper_with_buffer_provider , try_new_with_mapper_unstable , Self ,]) ; # [doc = " A constructor which creates a [`CaseMapCloser`] from an existing [`CaseMapper`]"] # [doc = " (either owned or as a reference)"] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [cfg (feature = "compiled_data")] pub const fn new_with_mapper (casemapper : CM) -> Self { Self { cm : casemapper , unfold : DataPayload :: from_static_ref (crate :: provider :: Baked :: SINGLETON_CASE_MAP_UNFOLD_V1 ,) , } } # [doc = " Construct this object to wrap an existing CaseMapper (or a reference to one), loading additional data as needed."] # [doc = icu_provider :: gen_buffer_unstable_docs ! (UNSTABLE , Self :: new_with_mapper)] pub fn try_new_with_mapper_unstable < P > (provider : & P , casemapper : CM) -> Result < Self , DataError > where P : DataProvider < CaseMapV1 > + DataProvider < CaseMapUnfoldV1 > + ? Sized , { let unfold = provider . load (Default :: default ()) ? . payload ; Ok (Self { cm : casemapper , unfold , }) } # [doc = " Constructs a borrowed version of this type for more efficient querying."] pub fn as_borrowed (& self) -> CaseMapCloserBorrowed < '_ > { CaseMapCloserBorrowed { cm : self . cm . as_ref () . as_borrowed () , unfold : self . unfold . get () , } } }
};
}
