// Generated macro for PluralRules (struct)
macro_rules! DepcratePluralRules {
() => {
// Module: crate
// Provides: {"PluralRules"}
// Dependencies: {}
# [doc = " A struct which provides an ability to retrieve an appropriate"] # [doc = " [`Plural Category`] for a given number."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::locale;"] # [doc = " use icu::plurals::{PluralCategory, PluralRules};"] # [doc = ""] # [doc = " let pr = PluralRules::try_new(locale!(\"en\").into(), Default::default())"] # [doc = "     .expect(\"locale should be present\");"] # [doc = ""] # [doc = " assert_eq!(pr.category_for(5_usize), PluralCategory::Other);"] # [doc = " ```"] # [doc = ""] # [doc = " [`ICU4X`]: ../icu/index.html"] # [doc = " [`Plural Type`]: PluralRuleType"] # [doc = " [`Plural Category`]: PluralCategory"] # [derive (Debug)] pub struct PluralRules (DataPayload < ErasedMarker < PluralRulesData < 'static > > >) ;
};
}
