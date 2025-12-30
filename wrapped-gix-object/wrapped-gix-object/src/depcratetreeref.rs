// Generated macro for TreeRef (struct)
macro_rules! DepcrateTreeRef {
() => {
// Module: crate
// Provides: {"TreeRef"}
// Dependencies: {}
# [doc = " A directory snapshot containing files (blobs), directories (trees) and submodules (commits)."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct TreeRef < 'a > { # [doc = " The directories and files contained in this tree."] # [doc = ""] # [doc = " Beware that the sort order isn't *quite* by name, so one may bisect only with a [`tree::EntryRef`] to handle ordering correctly."] # [cfg_attr (feature = "serde" , serde (borrow))] pub entries : Vec < tree :: EntryRef < 'a > > , }
};
}
