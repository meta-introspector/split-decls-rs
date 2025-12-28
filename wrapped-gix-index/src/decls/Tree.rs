macro_rules! Tree {
    () => {
        # [doc = " A structure to associate object ids of a tree with sections in the index entries list."] # [doc = ""] # [doc = " It allows to more quickly build trees by avoiding as it can quickly reuse portions of the index and its associated tree ids"] # [doc = " if there was no change to them. Portions of this tree are invalidated as the index is changed."] # [derive (PartialEq , Eq , Clone , Debug)] pub struct Tree { # [doc = " The name of the tree/directory, or empty if it's the root tree."] pub name : SmallVec < u8 , 23 > , # [doc = " The id of the directory tree of the associated tree object."] pub id : gix_hash :: ObjectId , # [doc = " The amount of non-tree items in this directory tree, including sub-trees, recursively."] # [doc = " The value of the top-level tree is thus equal to the value of the total amount of entries."] # [doc = " If `None`, the tree is considered invalid and needs to be refreshed"] pub num_entries : Option < u32 > , # [doc = " The child-trees below the current tree."] pub children : Vec < Tree > , }
    };
}

Tree!();