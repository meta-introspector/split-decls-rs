// Generated macro for get_special (function)
macro_rules! Depcrate_providerget_special {
() => {
// Module: crate::provider
// Provides: {"get_special"}
// Dependencies: {}
# [doc = " Helper function to access a value from [`PluralElementsTupleSliceVarULE`]"] fn get_special < V : VarULE + ? Sized > (data : & PluralElementsTupleSliceVarULE < V > , key : PluralElementsKeys ,) -> Option < (FourBitMetadata , & V) > { data . iter () . filter_map (| ule | { let PluralCategoryAndMetadata { plural_category , metadata , } = ule . sized . get () ; (plural_category == key) . then_some ((metadata , & ule . variable)) }) . next () }
};
}
