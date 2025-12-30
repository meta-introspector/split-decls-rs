// Generated macro for Platform (struct)
macro_rules! Depcrate_object_tree_diffPlatform {
() => {
// Module: crate::object::tree::diff
// Provides: {"Platform"}
// Dependencies: {}
# [doc = " The diffing platform returned by [`Tree::changes()`]."] # [derive (Clone)] pub struct Platform < 'a , 'repo > { state : gix_diff :: tree :: State , lhs : & 'a Tree < 'repo > , options : crate :: diff :: Options , }
};
}
