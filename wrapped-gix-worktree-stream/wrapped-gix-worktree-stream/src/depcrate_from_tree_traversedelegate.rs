// Generated macro for Delegate (struct)
macro_rules! Depcrate_from_tree_traverseDelegate {
() => {
// Module: crate::from_tree::traverse
// Provides: {"Delegate"}
// Dependencies: {}
pub struct Delegate < 'a , AttributesFn , Find > where Find : gix_object :: Find , { pub (crate) out : & 'a mut gix_features :: io :: pipe :: Writer , pub (crate) err : SharedErrorSlot , pub (crate) path_deque : VecDeque < BString > , pub (crate) path : BString , pub (crate) pipeline : gix_filter :: Pipeline , pub (crate) attrs : gix_attributes :: search :: Outcome , pub (crate) fetch_attributes : AttributesFn , pub (crate) objects : Find , pub (crate) buf : Vec < u8 > , }
};
}
