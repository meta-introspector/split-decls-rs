// Generated macro for impl_1873 (impl)
macro_rules! Depcrate_query_source_joinsimpl_1873 {
() => {
// Module: crate::query_source::joins
// Provides: {"impl_1873"}
// Dependencies: {}
impl < Left , Mid , Selection , Kind > AppendSelection < Selection > for Join < Left , Mid , Kind > where Left : QuerySource , Mid : QuerySource , Self : QuerySource , < Self as QuerySource > :: DefaultSelection : TupleAppend < Selection > , { type Output = < < Self as QuerySource > :: DefaultSelection as TupleAppend < Selection > > :: Output ; fn append_selection (& self , selection : Selection) -> Self :: Output { self . default_selection () . tuple_append (selection) } }
};
}
