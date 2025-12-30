// Generated macro for impl_201 (impl)
macro_rules! Depcrate_internalsimpl_201 {
() => {
// Module: crate::internals
// Provides: {"impl_201"}
// Dependencies: {}
impl CaseMapLocale { pub const fn from_langid (langid : & LanguageIdentifier) -> Self { use icu_locale_core :: subtags :: { language , Language } ; const TR : Language = language ! ("tr") ; const AZ : Language = language ! ("az") ; const LT : Language = language ! ("lt") ; const EL : Language = language ! ("el") ; const NL : Language = language ! ("nl") ; const HY : Language = language ! ("hy") ; match langid . language { TR | AZ => Self :: Turkish , LT => Self :: Lithuanian , EL => Self :: Greek , NL => Self :: Dutch , HY => Self :: Armenian , _ => Self :: Root , } } }
};
}
