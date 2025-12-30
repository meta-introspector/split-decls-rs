// Generated macro for Tree (struct)
macro_rules! Depcrate_nestedTree {
() => {
// Module: crate::nested
// Provides: {"Tree"}
// Dependencies: {}
# [doc = " An n-tree."] # [doc = ""] # [doc = " The algorithm used to generate a potentially deep object is to create a tree, one node at a"] # [doc = " time, where each node is put as a child of a random existing node in the tree."] struct Tree { # [doc = " The tree-view of the tree."] root : Rc < RefCell < Node > > , # [doc = " Array of all the nodes in the tree, including the root node."] nodes : Vec < Rc < RefCell < Node > > > , # [doc = " The RNG state."] # [doc = ""] # [doc = " We don't need to be cryptographically secure. [`SmallRng`] also implements the"] # [doc = " [`SeedableRng`] trait, allowing runs to be predictable."] rng : SmallRng , }
};
}
