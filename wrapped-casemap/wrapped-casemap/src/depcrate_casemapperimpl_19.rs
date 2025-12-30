// Generated macro for impl_19 (impl)
macro_rules! Depcrate_casemapperimpl_19 {
() => {
// Module: crate::casemapper
// Provides: {"impl_19"}
// Dependencies: {}
impl CaseMapper { # [doc = " Creates a [`CaseMapperBorrowed`] using compiled data."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use icu::casemap::CaseMapper;"] # [doc = " use icu::locale::langid;"] # [doc = ""] # [doc = " let cm = CaseMapper::new();"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     cm.uppercase_to_string(\"hello world\", &langid!(\"und\")),"] # [doc = "     \"HELLO WORLD\""] # [doc = " );"] # [doc = " ```"] # [cfg (feature = "compiled_data")] # [expect (clippy :: new_ret_no_self)] pub const fn new () -> CaseMapperBorrowed < 'static > { CaseMapperBorrowed :: new () } # [doc = " Constructs a borrowed version of this type for more efficient querying."] pub fn as_borrowed (& self) -> CaseMapperBorrowed < '_ > { CaseMapperBorrowed { data : self . data . get () , } } icu_provider :: gen_buffer_data_constructors ! (() -> error : DataError , functions : [new : skip , try_new_with_buffer_provider , try_new_unstable , Self ,]) ; # [doc = icu_provider :: gen_buffer_unstable_docs ! (UNSTABLE , Self :: new)] pub fn try_new_unstable < P > (provider : & P) -> Result < CaseMapper , DataError > where P : DataProvider < CaseMapV1 > + ? Sized , { let data = provider . load (Default :: default ()) ? . payload ; Ok (Self { data }) } }
};
}
