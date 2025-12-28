macro_rules! deps {
    () => {
        Tag!();
        Tree!();
        Commit!();
        Blob!();
    };
}

macro_rules! Kind {
    () => {
        deps!();
        # [doc = " The four types of objects that git differentiates."] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [allow (missing_docs)] pub enum Kind { Tree , Blob , Commit , Tag , }
    };
}

Kind!();