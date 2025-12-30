// Generated macro for impl_48 (impl)
macro_rules! Depcrateimpl_48 {
() => {
// Module: crate
// Provides: {"impl_48"}
// Dependencies: {}
impl Generate for Action { fn generate < R : rand :: Rng > (rng : & mut R) -> Self { use Action :: * ; if u64 :: generate (rng) % 1000 == 0 { DeployContract (DeployContractAction :: generate (rng)) } else { match u64 :: generate (rng) % 7 { 0 => CreateAccount (CreateAccountAction :: generate (rng)) , 1 => FunctionCall (FunctionCallAction :: generate (rng)) , 2 => Transfer (TransferAction :: generate (rng)) , 3 => Stake (StakeAction :: generate (rng)) , 4 => AddKey (AddKeyAction :: generate (rng)) , 5 => DeleteKey (DeleteKeyAction :: generate (rng)) , 6 => DeleteAccount (DeleteAccountAction :: generate (rng)) , _ => unimplemented ! () , } } } }
};
}
