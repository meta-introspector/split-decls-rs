// Generated macro for tests (module)
macro_rules! Depcrate_state_traitstests {
() => {
// Module: crate::state_traits
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use { super :: * , solana_pubkey :: Pubkey } ; # [test] fn test_account_state () { let state = 42u64 ; assert ! (AccountSharedData :: default () . set_state (& state) . is_err ()) ; let res = AccountSharedData :: default () . state () as Result < u64 , InstructionError > ; assert ! (res . is_err ()) ; let mut account = AccountSharedData :: new (0 , std :: mem :: size_of :: < u64 > () , & Pubkey :: default ()) ; assert ! (account . set_state (& state) . is_ok ()) ; let stored_state : u64 = account . state () . unwrap () ; assert_eq ! (stored_state , state) ; } }
};
}
