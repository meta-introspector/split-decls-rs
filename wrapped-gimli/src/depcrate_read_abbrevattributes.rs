// Generated macro for Attributes (enum)
macro_rules! Depcrate_read_abbrevAttributes {
() => {
// Module: crate::read::abbrev
// Provides: {"Attributes"}
// Dependencies: {}
# [doc = " A list of attributes found in an `Abbreviation`"] # [derive (Clone)] pub (crate) enum Attributes { Inline { buf : [AttributeSpecification ; MAX_ATTRIBUTES_INLINE] , len : usize , } , Heap (Vec < AttributeSpecification >) , }
};
}
