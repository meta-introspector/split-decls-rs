macro_rules! deps {
    () => {
        EntriesTreeNode!();
        EntriesTree!();
        EntriesTreeIter!();
        Result!();
        Reader!();
    };
}

macro_rules! impl_651 {
    () => {
        deps!();
        impl < 'abbrev , 'unit , 'tree , R : Reader > EntriesTreeIter < 'abbrev , 'unit , 'tree , R > { fn new (tree : & 'tree mut EntriesTree < 'abbrev , 'unit , R > , depth : isize ,) -> EntriesTreeIter < 'abbrev , 'unit , 'tree , R > { EntriesTreeIter { tree , depth , empty : false , } } # [doc = " Returns an `EntriesTreeNode` for the next child entry."] # [doc = ""] # [doc = " Returns `None` if there are no more children."] pub fn next < 'me > (& 'me mut self) -> Result < Option < EntriesTreeNode < 'abbrev , 'unit , 'me , R > > > { if self . empty { Ok (None) } else if self . tree . next (self . depth) ? { Ok (Some (EntriesTreeNode :: new (self . tree , self . depth + 1))) } else { self . empty = true ; Ok (None) } } }
    };
}

impl_651!();