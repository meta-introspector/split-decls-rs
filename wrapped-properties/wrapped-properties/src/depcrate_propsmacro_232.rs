// Generated macro for macro_232 (macro)
macro_rules! Depcrate_propsmacro_232 {
() => {
// Module: crate::props
// Provides: {"macro_232"}
// Dependencies: {}
make_binary_property ! { name : "Ids_Trinary_Operator" ; short_name : "IDST" ; ident : IdsTrinaryOperator ; data_marker : crate :: provider :: PropertyBinaryIdsTrinaryOperatorV1 ; singleton : SINGLETON_PROPERTY_BINARY_IDS_TRINARY_OPERATOR_V1 ; # [doc = " Characters used in Ideographic Description Sequences."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::IdsTrinaryOperator;"] # [doc = ""] # [doc = " let ids_trinary_operator = CodePointSetData::new::<IdsTrinaryOperator>();"] # [doc = ""] # [doc = " assert!(ids_trinary_operator.contains('\\u{2FF2}'));  // IDEOGRAPHIC DESCRIPTION CHARACTER LEFT TO MIDDLE AND RIGHT"] # [doc = " assert!(ids_trinary_operator.contains('\\u{2FF3}'));  // IDEOGRAPHIC DESCRIPTION CHARACTER ABOVE TO MIDDLE AND BELOW"] # [doc = " assert!(!ids_trinary_operator.contains('\\u{2FF4}'));"] # [doc = " assert!(!ids_trinary_operator.contains('\\u{2FF5}'));  // IDEOGRAPHIC DESCRIPTION CHARACTER SURROUND FROM ABOVE"] # [doc = " assert!(!ids_trinary_operator.contains('\\u{3006}'));  // IDEOGRAPHIC CLOSING MARK"] # [doc = " ```"] }
};
}
