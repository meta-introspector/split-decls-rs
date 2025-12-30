// Generated macro for macro_262 (macro)
macro_rules! Depcrate_propsmacro_262 {
() => {
// Module: crate::props
// Provides: {"macro_262"}
// Dependencies: {}
make_binary_property ! { name : "XID_Start" ; short_name : "XIDS" ; ident : XidStart ; data_marker : crate :: provider :: PropertyBinaryXidStartV1 ; singleton : SINGLETON_PROPERTY_BINARY_XID_START_V1 ; # [doc = " Characters that can begin an identifier."] # [doc = ""] # [doc = " See [`Unicode"] # [doc = " Standard Annex #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for more"] # [doc = " details."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::XidStart;"] # [doc = ""] # [doc = " let xid_start = CodePointSetData::new::<XidStart>();"] # [doc = ""] # [doc = " assert!(xid_start.contains('x'));"] # [doc = " assert!(!xid_start.contains('1'));"] # [doc = " assert!(!xid_start.contains('_'));"] # [doc = " assert!(xid_start.contains('ߝ'));  // U+07DD NKO LETTER FA"] # [doc = " assert!(!xid_start.contains('ⓧ'));  // U+24E7 CIRCLED LATIN SMALL LETTER X"] # [doc = " assert!(!xid_start.contains('\\u{FC5E}'));  // ARABIC LIGATURE SHADDA WITH DAMMATAN ISOLATED FORM"] # [doc = " ```"] }
};
}
