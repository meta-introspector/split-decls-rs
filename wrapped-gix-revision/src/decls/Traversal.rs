macro_rules! Traversal {
    () => {
        # [doc = " Define how to traverse the commit graph."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] pub enum Traversal { # [doc = " Select the given parent commit of the currently selected commit, start at `1` for the first parent."] # [doc = " The value will never be `0`."] NthParent (usize) , # [doc = " Select the given ancestor of the currently selected commit, start at `1` for the first ancestor."] # [doc = " The value will never be `0`."] NthAncestor (usize) , }
    };
}

Traversal!();