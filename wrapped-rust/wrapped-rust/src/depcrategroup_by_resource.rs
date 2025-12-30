// Generated macro for group_by_resource (function)
macro_rules! Depcrategroup_by_resource {
() => {
// Module: crate
// Provides: {"group_by_resource"}
// Dependencies: {}
fn group_by_resource < 'a > (funcs : impl Iterator < Item = & 'a Function > ,) -> BTreeMap < Option < TypeId > , Vec < & 'a Function > > { let mut by_resource = BTreeMap :: < _ , Vec < _ > > :: new () ; for func in funcs { by_resource . entry (func . kind . resource ()) . or_default () . push (func) ; } by_resource }
};
}
