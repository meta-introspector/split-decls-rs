// Generated macro for Update (enum)
macro_rules! DepcrateUpdate {
() => {
// Module: crate
// Provides: {"Update"}
// Dependencies: {}
# [doc = " An instruction on how to"] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum Update { # [doc = " Shallow the given `id`."] Shallow (gix_hash :: ObjectId) , # [doc = " Don't shallow the given `id` anymore."] Unshallow (gix_hash :: ObjectId) , }
};
}
