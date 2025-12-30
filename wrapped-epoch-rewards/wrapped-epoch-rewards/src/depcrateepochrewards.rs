// Generated macro for EpochRewards (struct)
macro_rules! DepcrateEpochRewards {
() => {
// Module: crate
// Provides: {"EpochRewards"}
// Dependencies: {}
# [repr (C , align (16))] # [cfg_attr (feature = "frozen-abi" , derive (solana_frozen_abi_macro :: AbiExample))] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (Debug , PartialEq , Eq , Default , CloneZeroed)] pub struct EpochRewards { # [doc = " The starting block height of the rewards distribution in the current"] # [doc = " epoch"] pub distribution_starting_block_height : u64 , # [doc = " Number of partitions in the rewards distribution in the current epoch,"] # [doc = " used to generate an EpochRewardsHasher"] pub num_partitions : u64 , # [doc = " The blockhash of the parent block of the first block in the epoch, used"] # [doc = " to seed an EpochRewardsHasher"] pub parent_blockhash : Hash , # [doc = " The total rewards points calculated for the current epoch, where points"] # [doc = " equals the sum of (delegated stake * credits observed) for all"] # [doc = " delegations"] pub total_points : u128 , # [doc = " The total rewards calculated for the current epoch. This may be greater"] # [doc = " than the total `distributed_rewards` at the end of the rewards period,"] # [doc = " due to rounding and inability to deliver rewards smaller than 1 lamport."] pub total_rewards : u64 , # [doc = " The rewards currently distributed for the current epoch, in lamports"] pub distributed_rewards : u64 , # [doc = " Whether the rewards period (including calculation and distribution) is"] # [doc = " active"] pub active : bool , }
};
}
