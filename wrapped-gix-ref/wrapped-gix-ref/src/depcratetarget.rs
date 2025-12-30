// Generated macro for Target (enum)
macro_rules! DepcrateTarget {
() => {
// Module: crate
// Provides: {"Target"}
// Dependencies: {}
# [doc = " Denotes a ref target, equivalent to [`Kind`], but with mutable data."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum Target { # [doc = " A ref that points directly to an object id."] Object (ObjectId) , # [doc = " A ref that points to another reference by its validated name, adding a level of indirection."] # [doc = ""] # [doc = " Note that this is an extension of gitoxide which will be helpful in logging all reference changes."] Symbolic (FullName) , }
};
}
