// Generated macro for impl_5 (impl)
macro_rules! Depcrateimpl_5 {
() => {
// Module: crate
// Provides: {"impl_5"}
// Dependencies: {}
impl fmt :: Display for RewardType { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , match self { RewardType :: Fee => "fee" , RewardType :: Rent => "rent" , RewardType :: Staking => "staking" , RewardType :: Voting => "voting" , }) } }
};
}
