// Generated macro for macro_235 (macro)
macro_rules! Depcrate_propsmacro_235 {
() => {
// Module: crate::props
// Provides: {"macro_235"}
// Dependencies: {}
make_binary_property ! { name : "Logical_Order_Exception" ; short_name : "LOE" ; ident : LogicalOrderException ; data_marker : crate :: provider :: PropertyBinaryLogicalOrderExceptionV1 ; singleton : SINGLETON_PROPERTY_BINARY_LOGICAL_ORDER_EXCEPTION_V1 ; # [doc = " A small number of spacing vowel letters occurring in certain Southeast Asian scripts such as Thai and Lao."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::LogicalOrderException;"] # [doc = ""] # [doc = " let logical_order_exception = CodePointSetData::new::<LogicalOrderException>();"] # [doc = ""] # [doc = " assert!(logical_order_exception.contains('ແ'));  // U+0EC1 LAO VOWEL SIGN EI"] # [doc = " assert!(!logical_order_exception.contains('ະ'));  // U+0EB0 LAO VOWEL SIGN A"] # [doc = " ```"] }
};
}
