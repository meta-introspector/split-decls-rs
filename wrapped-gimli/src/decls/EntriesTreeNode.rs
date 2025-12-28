macro_rules! deps {
    () => {
        Reader!();
        EntriesTree!();
    };
}

macro_rules! EntriesTreeNode {
    () => {
        deps!();
        # [doc = " A node in the Debugging Information Entry tree."] # [doc = ""] # [doc = " The root node of a tree can be obtained"] # [doc = " via [`EntriesTree::root`](./struct.EntriesTree.html#method.root)."] # [derive (Debug)] pub struct EntriesTreeNode < 'abbrev , 'unit , 'tree , R : Reader > { tree : & 'tree mut EntriesTree < 'abbrev , 'unit , R > , depth : isize , }
    };
}

EntriesTreeNode!();