// Generated macro for impl_1890 (impl)
macro_rules! Depcrate_query_source_joinsimpl_1890 {
() => {
// Module: crate::query_source::joins
// Provides: {"impl_1890"}
// Dependencies: {}
impl < Left , Right , Kind > ToInnerJoin for Join < Left , Right , Kind > where Left : ToInnerJoin + QuerySource , Left :: InnerJoin : QuerySource , Right : ToInnerJoin + QuerySource , Right :: InnerJoin : QuerySource , { type InnerJoin = Join < Left :: InnerJoin , Right :: InnerJoin , Inner > ; }
};
}
