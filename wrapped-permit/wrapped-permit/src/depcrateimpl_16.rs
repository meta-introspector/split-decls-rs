// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
impl Inner { # [must_use] pub fn new (revoked : bool) -> Self { Inner { revoked , opt_waker : None , subs : HashSet :: new () , } } pub fn add_sub (& mut self , node : & Arc < Node >) { if ! self . revoked { self . subs . insert (ArcNode (Arc :: clone (node))) ; } } pub fn remove_sub (& mut self , node : & Arc < Node >) { let arc_node = ArcNode (Arc :: clone (node)) ; self . subs . remove (& arc_node) ; } pub fn poll (& mut self , cx : & mut Context < '_ >) -> Poll < () > { if self . revoked { Poll :: Ready (()) } else { self . opt_waker = Some (cx . waker () . clone ()) ; Poll :: Pending } } pub fn revoke (& mut self) -> (Option < Waker > , Vec < ArcNode >) { self . revoked = true ; (self . opt_waker . take () , self . subs . iter () . cloned () . collect ()) } }
};
}
