// Generated macro for Membership (struct)
macro_rules! Depcrate_membershipMembership {
() => {
// Module: crate::membership
// Provides: {"Membership"}
// Dependencies: {}
# [derive (bincode_2 :: Encode , bincode_2 :: Decode , serde :: Serialize , serde :: Deserialize , Debug , PartialEq , Eq ,)] # [bincode (crate = "bincode_2")] pub struct Membership { # [doc = " learners set"] learners : BTreeSet < NodeId > , # [doc = " Multi configs."] configs : Vec < BTreeSet < NodeId > > , # [doc = " Cache of all node ids."] all_members : BTreeSet < NodeId > , }
};
}
