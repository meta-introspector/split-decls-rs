// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl EpochRewards { pub fn distribute (& mut self , amount : u64) { let new_distributed_rewards = self . distributed_rewards . saturating_add (amount) ; assert ! (new_distributed_rewards <= self . total_rewards) ; self . distributed_rewards = new_distributed_rewards ; } }
};
}
