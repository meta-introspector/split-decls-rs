// Generated macro for macro_251 (macro)
macro_rules! Depcrate_propsmacro_251 {
() => {
// Module: crate::props
// Provides: {"macro_251"}
// Dependencies: {}
make_binary_property ! { name : "Soft_Dotted" ; short_name : "SD" ; ident : SoftDotted ; data_marker : crate :: provider :: PropertyBinarySoftDottedV1 ; singleton : SINGLETON_PROPERTY_BINARY_SOFT_DOTTED_V1 ; # [doc = " Characters with a \"soft dot\", like i or j."] # [doc = ""] # [doc = " An accent placed on these characters causes"] # [doc = " the dot to disappear."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::SoftDotted;"] # [doc = ""] # [doc = " let soft_dotted = CodePointSetData::new::<SoftDotted>();"] # [doc = ""] # [doc = " assert!(soft_dotted.contains('і'));  //U+0456 CYRILLIC SMALL LETTER BYELORUSSIAN-UKRAINIAN I"] # [doc = " assert!(!soft_dotted.contains('ı'));  // U+0131 LATIN SMALL LETTER DOTLESS I"] # [doc = " ```"] }
};
}
