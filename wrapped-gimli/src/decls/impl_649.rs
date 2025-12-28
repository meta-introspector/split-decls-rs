macro_rules! deps {
    () => {
        EntriesTree!();
        EntriesTreeIter!();
        Reader!();
        DebuggingInformationEntry!();
        EntriesTreeNode!();
    };
}

macro_rules! impl_649 {
    () => {
        deps!();
        impl < 'abbrev , 'unit , 'tree , R : Reader > EntriesTreeNode < 'abbrev , 'unit , 'tree , R > { fn new (tree : & 'tree mut EntriesTree < 'abbrev , 'unit , R > , depth : isize ,) -> EntriesTreeNode < 'abbrev , 'unit , 'tree , R > { debug_assert ! (tree . entry . is_some ()) ; EntriesTreeNode { tree , depth } } # [doc = " Returns the current entry in the tree."] pub fn entry (& self) -> & DebuggingInformationEntry < 'abbrev , 'unit , R > { self . tree . entry . as_ref () . unwrap () } # [doc = " Create an iterator for the children of the current entry."] # [doc = ""] # [doc = " The current entry can no longer be accessed after creating the"] # [doc = " iterator."] pub fn children (self) -> EntriesTreeIter < 'abbrev , 'unit , 'tree , R > { EntriesTreeIter :: new (self . tree , self . depth) } }
    };
}

impl_649!()