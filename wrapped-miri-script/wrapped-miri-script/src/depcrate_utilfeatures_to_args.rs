// Generated macro for features_to_args (function)
macro_rules! Depcrate_utilfeatures_to_args {
() => {
// Module: crate::util
// Provides: {"features_to_args"}
// Dependencies: {}
# [doc = " Turns a list of features into a list of arguments to pass to cargo invocations."] # [doc = " Each feature will go in its own argument, e.g. \"--features feat1 --features feat2\"."] fn features_to_args (features : & [String]) -> impl IntoIterator < Item = & str > { features . iter () . flat_map (| feat | ["--features" , feat]) }
};
}
