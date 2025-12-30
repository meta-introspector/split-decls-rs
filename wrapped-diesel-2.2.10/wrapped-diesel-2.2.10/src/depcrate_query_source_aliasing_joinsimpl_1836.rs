// Generated macro for impl_1836 (impl)
macro_rules! Depcrate_query_source_aliasing_joinsimpl_1836 {
() => {
// Module: crate::query_source::aliasing::joins
// Provides: {"impl_1836"}
// Dependencies: {}
impl < S , Selection > AppendSelection < Selection > for Alias < S > where Self : QuerySource , { type Output = (< Self as QuerySource > :: DefaultSelection , Selection) ; fn append_selection (& self , selection : Selection) -> Self :: Output { (self . default_selection () , selection) } }
};
}
