// Generated macro for Tuple (enum)
macro_rules! Depcrate_se_elementTuple {
() => {
// Module: crate::se::element
// Provides: {"Tuple"}
// Dependencies: {}
# [doc = " A serializer for tuple variants. Tuples can be serialized in two modes:"] # [doc = " - wrapping each tuple field into a tag"] # [doc = " - without wrapping, fields are delimited by a space"] pub enum Tuple < 'w , 'k , W : Write > { # [doc = " Serialize each tuple field as an element"] Element (ElementSerializer < 'w , 'k , W >) , # [doc = " Serialize tuple as an `xs:list`: space-delimited content of fields"] Text (SimpleSeq < & 'w mut W >) , }
};
}
