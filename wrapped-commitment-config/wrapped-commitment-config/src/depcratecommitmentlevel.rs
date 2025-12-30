// Generated macro for CommitmentLevel (enum)
macro_rules! DepcrateCommitmentLevel {
() => {
// Module: crate
// Provides: {"CommitmentLevel"}
// Dependencies: {}
# [cfg_attr (feature = "serde" , derive (serde_derive :: Serialize , serde_derive :: Deserialize) , serde (rename_all = "camelCase"))] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] # [doc = " An attribute of a slot. It describes how finalized a block is at some point in time. For example, a slot"] # [doc = " is said to be at the max level immediately after the cluster recognizes the block at that slot as"] # [doc = " finalized. When querying the ledger state, use lower levels of commitment to report progress and higher"] # [doc = " levels to ensure state changes will not be rolled back."] pub enum CommitmentLevel { # [doc = " The highest slot of the heaviest fork processed by the node. Ledger state at this slot is"] # [doc = " not derived from a confirmed or finalized block, but if multiple forks are present, is from"] # [doc = " the fork the validator believes is most likely to finalize."] Processed , # [doc = " The highest slot that has been voted on by supermajority of the cluster, ie. is confirmed."] # [doc = " Confirmation incorporates votes from gossip and replay. It does not count votes on"] # [doc = " descendants of a block, only direct votes on that block, and upholds \"optimistic"] # [doc = " confirmation\" guarantees in release 1.3 and onwards."] Confirmed , # [doc = " The highest slot having reached max vote lockout, as recognized by a supermajority of the"] # [doc = " cluster."] Finalized , }
};
}
