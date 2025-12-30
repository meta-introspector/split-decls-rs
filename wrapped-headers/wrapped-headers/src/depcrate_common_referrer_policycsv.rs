// Generated macro for csv (function)
macro_rules! Depcrate_common_referrer_policycsv {
() => {
// Module: crate::common::referrer_policy
// Provides: {"csv"}
// Dependencies: {}
fn csv < 'i , I > (values : I) -> impl Iterator < Item = & 'i str > where I : Iterator < Item = & 'i HeaderValue > , { values . flat_map (| value | { value . to_str () . into_iter () . flat_map (| string | { string . split (',') . filter_map (| x | match x . trim () { "" => None , y => Some (y) , }) }) }) }
};
}
