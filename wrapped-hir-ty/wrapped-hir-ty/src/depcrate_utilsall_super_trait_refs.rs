// Generated macro for all_super_trait_refs (function)
macro_rules! Depcrate_utilsall_super_trait_refs {
() => {
// Module: crate::utils
// Provides: {"all_super_trait_refs"}
// Dependencies: {}
# [doc = " Given a trait ref (`Self: Trait`), builds all the implied trait refs for"] # [doc = " super traits. The original trait ref will be included. So the difference to"] # [doc = " `all_super_traits` is that we keep track of type parameters; for example if"] # [doc = " we have `Self: Trait<u32, i32>` and `Trait<T, U>: OtherTrait<U>` we'll get"] # [doc = " `Self: OtherTrait<i32>`."] pub (super) fn all_super_trait_refs < T > (db : & dyn HirDatabase , trait_ref : TraitRef , cb : impl FnMut (TraitRef) -> Option < T > ,) -> Option < T > { let seen = iter :: once (trait_ref . trait_id) . collect () ; SuperTraits { db , seen , stack : vec ! [trait_ref] } . find_map (cb) }
};
}
