// Generated macro for LocaleDirectionality (struct)
macro_rules! Depcrate_directionalityLocaleDirectionality {
() => {
// Module: crate::directionality
// Provides: {"LocaleDirectionality"}
// Dependencies: {}
# [doc = " Provides methods to determine the direction of a locale."] # [doc = ""] # [doc = " The `Expander` generic parameter wraps a [`LocaleExpander`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::{langid, Direction, LocaleDirectionality};"] # [doc = ""] # [doc = " let ld = LocaleDirectionality::new_common();"] # [doc = ""] # [doc = " assert_eq!(ld.get(&langid!(\"en\")), Some(Direction::LeftToRight));"] # [doc = " ```"] # [derive (Debug)] pub struct LocaleDirectionality < Expander = LocaleExpander > { script_direction : DataPayload < LocaleScriptDirectionV1 > , expander : Expander , }
};
}
