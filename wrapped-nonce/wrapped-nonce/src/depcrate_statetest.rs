// Generated macro for test (module)
macro_rules! Depcrate_statetest {
() => {
// Module: crate::state
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use { super :: * , crate :: versions :: Versions } ; # [test] fn default_is_uninitialized () { assert_eq ! (State :: default () , State :: Uninitialized) } # [test] fn test_nonce_state_size () { let data = Versions :: new (State :: Initialized (Data :: default ())) ; let size = bincode :: serialized_size (& data) . unwrap () ; assert_eq ! (State :: size () as u64 , size) ; } }
};
}
