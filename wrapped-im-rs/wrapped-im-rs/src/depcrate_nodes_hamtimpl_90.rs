// Generated macro for impl_90 (impl)
macro_rules! Depcrate_nodes_hamtimpl_90 {
() => {
// Module: crate::nodes::hamt
// Provides: {"impl_90"}
// Dependencies: {}
impl < 'a , A > Iterator for IterMut < 'a , A > where A : Clone + 'a , { type Item = (& 'a mut A , HashBits) ; fn next (& mut self) -> Option < Self :: Item > { if self . count == 0 { return None ; } if self . collision . is_some () { if let Some ((hash , ref mut coll)) = self . collision { match coll . next () { None => { } Some (value) => { self . count -= 1 ; return Some ((value , hash)) ; } } } self . collision = None ; return self . next () ; } match self . current . next () { Some (Entry :: Value (value , hash)) => { self . count -= 1 ; Some ((value , * hash)) } Some (Entry :: Node (child_ref)) => { let child = PoolRef :: make_mut (& self . pool , child_ref) ; let current = mem :: replace (& mut self . current , child . data . iter_mut ()) ; self . stack . push (current) ; self . next () } Some (Entry :: Collision (coll_ref)) => { let coll = Ref :: make_mut (coll_ref) ; self . collision = Some ((coll . hash , coll . data . iter_mut ())) ; self . next () } None => match self . stack . pop () { None => None , Some (iter) => { self . current = iter ; self . next () } } , } } fn size_hint (& self) -> (usize , Option < usize >) { (self . count , Some (self . count)) } }
};
}
