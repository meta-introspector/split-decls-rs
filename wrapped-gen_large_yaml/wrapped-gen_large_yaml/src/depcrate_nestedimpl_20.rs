// Generated macro for impl_20 (impl)
macro_rules! Depcrate_nestedimpl_20 {
() => {
// Module: crate::nested
// Provides: {"impl_20"}
// Dependencies: {}
impl Tree { # [doc = " Create a new tree."] fn new () -> Self { let root = Node :: new_rc_refcell () ; Tree { root : root . clone () , nodes : vec ! [root] , rng : SmallRng :: seed_from_u64 (42) , } } # [doc = " Add a new node as a child of a random node in the tree."] fn push_node (& mut self) { let new_node = Node :: new_rc_refcell () ; let n_nodes = self . nodes . len () ; let parent = & mut self . nodes [self . rng . gen_range ((3 * n_nodes / 4) .. n_nodes)] ; (* * parent) . borrow_mut () . push_child (new_node . clone ()) ; self . nodes . push (new_node) ; } # [doc = " Write the YAML representation of the tree to `writer`."] fn write_to < W : std :: io :: Write > (& self , writer : & mut W) -> std :: io :: Result < () > { (* self . root) . borrow () . write_to (writer , 0) } }
};
}
