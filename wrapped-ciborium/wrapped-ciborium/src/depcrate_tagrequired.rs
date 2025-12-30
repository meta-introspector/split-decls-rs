// Generated macro for Required (type)
macro_rules! Depcrate_tagRequired {
() => {
// Module: crate::tag
// Provides: {"Required"}
// Dependencies: {}
# [doc = " Require an exact tag."] # [deprecated = "use RequireExact"] pub type Required < V , const TAG : u64 > = RequireExact < V , TAG > ;
};
}
