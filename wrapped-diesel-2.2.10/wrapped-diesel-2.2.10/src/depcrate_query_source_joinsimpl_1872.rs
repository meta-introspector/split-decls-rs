// Generated macro for impl_1872 (impl)
macro_rules! Depcrate_query_source_joinsimpl_1872 {
() => {
// Module: crate::query_source::joins
// Provides: {"impl_1872"}
// Dependencies: {}
impl < T : Table , Selection > AppendSelection < Selection > for T { type Output = (T :: AllColumns , Selection) ; fn append_selection (& self , selection : Selection) -> Self :: Output { (T :: all_columns () , selection) } }
};
}
