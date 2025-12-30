// Generated macro for IdMap (type)
macro_rules! Depcrate_graphIdMap {
() => {
// Module: crate::graph
// Provides: {"IdMap"}
// Dependencies: {}
# [doc = " A mapping between an object id and arbitrary data, and produced when calling [`Graph::detach()`]."] pub type IdMap < T > = gix_hashtable :: HashMap < gix_hash :: ObjectId , T > ;
};
}
