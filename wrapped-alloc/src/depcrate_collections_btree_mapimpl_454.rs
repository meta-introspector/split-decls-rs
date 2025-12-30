// Generated macro for impl_454 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_454 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_454"}
// Dependencies: {}
impl < 'a , K , V , R > ExtractIfInner < 'a , K , V , R > { # [doc = " Allow Debug implementations to predict the next element."] pub (super) fn peek (& self) -> Option < (& K , & V) > { let edge = self . cur_leaf_edge . as_ref () ? ; edge . reborrow () . next_kv () . ok () . map (Handle :: into_kv) } # [doc = " Implementation of a typical `ExtractIf::next` method, given the predicate."] pub (super) fn next < F , A : Allocator + Clone > (& mut self , pred : & mut F , alloc : A) -> Option < (K , V) > where K : PartialOrd , R : RangeBounds < K > , F : FnMut (& K , & mut V) -> bool , { while let Ok (mut kv) = self . cur_leaf_edge . take () ? . next_kv () { let (k , v) = kv . kv_mut () ; match self . range . end_bound () { Bound :: Included (ref end) if (* k) . le (end) => () , Bound :: Excluded (ref end) if (* k) . lt (end) => () , Bound :: Unbounded => () , _ => return None , } if pred (k , v) { * self . length -= 1 ; let (kv , pos) = kv . remove_kv_tracking (| | { let root = unsafe { self . dormant_root . take () . unwrap () . awaken () } ; root . pop_internal_level (alloc . clone ()) ; self . dormant_root = Some (DormantMutRef :: new (root) . 1) ; } , alloc . clone () ,) ; self . cur_leaf_edge = Some (pos) ; return Some (kv) ; } self . cur_leaf_edge = Some (kv . next_leaf_edge ()) ; } None } # [doc = " Implementation of a typical `ExtractIf::size_hint` method."] pub (super) fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (* self . length)) } }
};
}
