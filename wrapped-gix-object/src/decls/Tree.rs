macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! Tree {
    () => {
        deps!();
        # [doc = " A mutable Tree, containing other trees, blobs or commits."] # [derive (Default , PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Tree { # [doc = " The directories and files contained in this tree. They must be and remain sorted by [`filename`][tree::Entry::filename]."] # [doc = ""] # [doc = " Beware that the sort order isn't *quite* by name, so one may bisect only with a [`tree::Entry`] to handle ordering correctly."] pub entries : Vec < tree :: Entry > , }
    };
}

Tree!();