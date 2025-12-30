// Generated macro for NonPrimary (struct)
macro_rules! Depcrate_elementsNonPrimary {
() => {
// Module: crate::elements
// Provides: {"NonPrimary"}
// Dependencies: {}
# [doc = " The purpose of grouping the non-primary bits"] # [doc = " into a struct is to allow for a future optimization"] # [doc = " that specializes code over whether storage for primary"] # [doc = " weights is needed or not. (I.e. whether to specialize"] # [doc = " on `CollationElement` or `NonPrimary`.)"] # [derive (Copy , Clone , PartialEq , Debug)] pub (crate) struct NonPrimary (u32) ;
};
}
