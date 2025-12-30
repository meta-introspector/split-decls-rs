// Generated macro for ScriptDisplayNames (struct)
macro_rules! Depcrate_displaynames_displaynamesScriptDisplayNames {
() => {
// Module: crate::displaynames::displaynames
// Provides: {"ScriptDisplayNames"}
// Dependencies: {}
# [doc = " Lookup of the locale-specific display names by script code."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::experimental::displaynames::{"] # [doc = "     DisplayNamesOptions, ScriptDisplayNames,"] # [doc = " };"] # [doc = " use icu::locale::{locale, subtags::script};"] # [doc = ""] # [doc = " let locale = locale!(\"en-001\").into();"] # [doc = " let options: DisplayNamesOptions = Default::default();"] # [doc = " let display_name = ScriptDisplayNames::try_new(locale, options)"] # [doc = "     .expect(\"Data should load successfully\");"] # [doc = ""] # [doc = " assert_eq!(display_name.of(script!(\"Maya\")), Some(\"Mayan hieroglyphs\"));"] # [doc = " ```"] # [derive (Default)] pub struct ScriptDisplayNames { options : DisplayNamesOptions , script_data : DataPayload < ScriptDisplayNamesV1 > , }
};
}
