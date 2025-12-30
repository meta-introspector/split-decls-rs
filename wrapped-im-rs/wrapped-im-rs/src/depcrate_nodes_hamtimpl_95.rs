// Generated macro for impl_95 (impl)
macro_rules! Depcrate_nodes_hamtimpl_95 {
() => {
// Module: crate::nodes::hamt
// Provides: {"impl_95"}
// Dependencies: {}
impl < A > Iterator for Drain < A > where A : HashValue + Clone , { type Item = (A , HashBits) ; fn next (& mut self) -> Option < Self :: Item > { if self . count == 0 { return None ; } if self . collision . is_some () { if let Some (ref mut coll) = self . collision { if let Some (value) = coll . data . pop () { self . count -= 1 ; return Some ((value , coll . hash)) ; } } self . collision = None ; return self . next () ; } match PoolRef :: make_mut (& self . pool , & mut self . current) . data . pop () { Some (Entry :: Value (value , hash)) => { self . count -= 1 ; Some ((value , hash)) } Some (Entry :: Collision (coll_ref)) => { self . collision = Some (clone_ref (coll_ref)) ; self . next () } Some (Entry :: Node (child)) => { let parent = mem :: replace (& mut self . current , child) ; self . stack . push (parent) ; self . next () } None => match self . stack . pop () { None => None , Some (parent) => { self . current = parent ; self . next () } } , } } fn size_hint (& self) -> (usize , Option < usize >) { (self . count , Some (self . count)) } }
};
}
