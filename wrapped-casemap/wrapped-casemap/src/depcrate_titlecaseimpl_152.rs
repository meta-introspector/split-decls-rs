// Generated macro for impl_152 (impl)
macro_rules! Depcrate_titlecaseimpl_152 {
() => {
// Module: crate::titlecase
// Provides: {"impl_152"}
// Dependencies: {}
impl TitlecaseMapperBorrowed < 'static > { # [doc = " A constructor which creates a [`TitlecaseMapperBorrowed`] using compiled data"] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [cfg (feature = "compiled_data")] pub const fn new () -> Self { Self { cm : CaseMapper :: new () , gc : icu_properties :: CodePointMapData :: < icu_properties :: props :: GeneralCategory > :: new () , } } # [doc = " Cheaply converts a [`TitlecaseMapperBorrowed<'static>`] into a [`TitlecaseMapper`]."] # [doc = ""] # [doc = " Note: Due to branching and indirection, using [`TitlecaseMapper`] might inhibit some"] # [doc = " compile-time optimizations that are possible with [`TitlecaseMapper`]."] pub const fn static_to_owned (self) -> TitlecaseMapper < CaseMapper > { TitlecaseMapper { cm : self . cm . static_to_owned () , gc : self . gc . static_to_owned () , } } }
};
}
