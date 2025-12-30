// Generated macro for by_resource (function)
macro_rules! Depcrate_world_generatorby_resource {
() => {
// Module: crate::world_generator
// Provides: {"by_resource"}
// Dependencies: {}
# [doc = " Group the specified functions by resource (or `None` for freestanding functions)."] # [doc = ""] # [doc = " The returned map is constructed by iterating over `funcs`, then iterating over `all_resources`, thereby"] # [doc = " ensuring that even resources with no associated functions will be represented in the result."] fn by_resource < 'a > (funcs : impl Iterator < Item = (& 'a str , & 'a Function) > , all_resources : impl Iterator < Item = TypeId > ,) -> IndexMap < Option < TypeId > , Vec < & 'a Function > > { let mut by_resource = IndexMap :: < _ , Vec < _ > > :: new () ; for (_ , func) in funcs { by_resource . entry (func . kind . resource ()) . or_default () . push (func) ; } for id in all_resources { by_resource . entry (Some (id)) . or_default () ; } by_resource }
};
}
