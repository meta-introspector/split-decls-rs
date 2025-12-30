// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; impl EpochRewards { pub fn new (total_rewards : u64 , distributed_rewards : u64 , distribution_starting_block_height : u64 ,) -> Self { Self { total_rewards , distributed_rewards , distribution_starting_block_height , .. Self :: default () } } } # [test] fn test_epoch_rewards_new () { let epoch_rewards = EpochRewards :: new (100 , 0 , 64) ; assert_eq ! (epoch_rewards . total_rewards , 100) ; assert_eq ! (epoch_rewards . distributed_rewards , 0) ; assert_eq ! (epoch_rewards . distribution_starting_block_height , 64) ; } # [test] fn test_epoch_rewards_distribute () { let mut epoch_rewards = EpochRewards :: new (100 , 0 , 64) ; epoch_rewards . distribute (100) ; assert_eq ! (epoch_rewards . total_rewards , 100) ; assert_eq ! (epoch_rewards . distributed_rewards , 100) ; } # [test] # [should_panic (expected = "new_distributed_rewards <= self.total_rewards")] fn test_epoch_rewards_distribute_panic () { let mut epoch_rewards = EpochRewards :: new (100 , 0 , 64) ; epoch_rewards . distribute (200) ; } }
};
}
