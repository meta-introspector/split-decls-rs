// Generated macro for tests (module)
macro_rules! Depcrate_statetests {
() => {
// Module: crate::state
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use { super :: * , bincode :: serialized_size } ; # [test] fn test_state_size_of_uninitialized () { let buffer_state = UpgradeableLoaderState :: Uninitialized ; let size = serialized_size (& buffer_state) . unwrap () ; assert_eq ! (UpgradeableLoaderState :: size_of_uninitialized () as u64 , size) ; } # [test] fn test_state_size_of_buffer_metadata () { let buffer_state = UpgradeableLoaderState :: Buffer { authority_address : Some (Pubkey :: default ()) , } ; let size = serialized_size (& buffer_state) . unwrap () ; assert_eq ! (UpgradeableLoaderState :: size_of_buffer_metadata () as u64 , size) ; } # [test] fn test_state_size_of_programdata_metadata () { let programdata_state = UpgradeableLoaderState :: ProgramData { upgrade_authority_address : Some (Pubkey :: default ()) , slot : 0 , } ; let size = serialized_size (& programdata_state) . unwrap () ; assert_eq ! (UpgradeableLoaderState :: size_of_programdata_metadata () as u64 , size) ; } # [test] fn test_state_size_of_program () { let program_state = UpgradeableLoaderState :: Program { programdata_address : Pubkey :: default () , } ; let size = serialized_size (& program_state) . unwrap () ; assert_eq ! (UpgradeableLoaderState :: size_of_program () as u64 , size) ; } }
};
}
