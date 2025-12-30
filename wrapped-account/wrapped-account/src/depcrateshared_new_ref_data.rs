// Generated macro for shared_new_ref_data (function)
macro_rules! Depcrateshared_new_ref_data {
() => {
// Module: crate
// Provides: {"shared_new_ref_data"}
// Dependencies: {}
# [cfg (feature = "bincode")] fn shared_new_ref_data < T : serde :: Serialize , U : WritableAccount > (lamports : u64 , state : & T , owner : & Pubkey ,) -> Result < RefCell < U > , bincode :: Error > { Ok (RefCell :: new (shared_new_data :: < T , U > (lamports , state , owner ,) ?)) }
};
}
