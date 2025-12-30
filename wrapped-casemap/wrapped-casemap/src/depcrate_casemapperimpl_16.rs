// Generated macro for impl_16 (impl)
macro_rules! Depcrate_casemapperimpl_16 {
() => {
// Module: crate::casemapper
// Provides: {"impl_16"}
// Dependencies: {}
impl CaseMapperBorrowed < 'static > { # [doc = " Cheaply converts a [`CaseMapperBorrowed<'static>`] into a [`CaseMapper`]."] # [doc = ""] # [doc = " Note: Due to branching and indirection, using [`CaseMapper`] might inhibit some"] # [doc = " compile-time optimizations that are possible with [`CaseMapperBorrowed`]."] pub const fn static_to_owned (self) -> CaseMapper { CaseMapper { data : DataPayload :: from_static_ref (self . data) , } } # [doc = " Creates a [`CaseMapperBorrowed`] using compiled data."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use icu::casemap::CaseMapper;"] # [doc = " use icu::locale::langid;"] # [doc = ""] # [doc = " let cm = CaseMapper::new();"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     cm.uppercase_to_string(\"hello world\", &langid!(\"und\")),"] # [doc = "     \"HELLO WORLD\""] # [doc = " );"] # [doc = " ```"] # [cfg (feature = "compiled_data")] pub const fn new () -> Self { Self { data : crate :: provider :: Baked :: SINGLETON_CASE_MAP_V1 , } } }
};
}
