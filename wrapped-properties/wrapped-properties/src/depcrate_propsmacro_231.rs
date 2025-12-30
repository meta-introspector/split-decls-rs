// Generated macro for macro_231 (macro)
macro_rules! Depcrate_propsmacro_231 {
() => {
// Module: crate::props
// Provides: {"macro_231"}
// Dependencies: {}
make_binary_property ! { name : "Ids_Binary_Operator" ; short_name : "IDSB" ; ident : IdsBinaryOperator ; data_marker : crate :: provider :: PropertyBinaryIdsBinaryOperatorV1 ; singleton : SINGLETON_PROPERTY_BINARY_IDS_BINARY_OPERATOR_V1 ; # [doc = " Characters used in Ideographic Description Sequences."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::IdsBinaryOperator;"] # [doc = ""] # [doc = " let ids_binary_operator = CodePointSetData::new::<IdsBinaryOperator>();"] # [doc = ""] # [doc = " assert!(ids_binary_operator.contains('\\u{2FF5}'));  // IDEOGRAPHIC DESCRIPTION CHARACTER SURROUND FROM ABOVE"] # [doc = " assert!(!ids_binary_operator.contains('\\u{3006}'));  // IDEOGRAPHIC CLOSING MARK"] # [doc = " ```"] }
};
}
