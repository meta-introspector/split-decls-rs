// Generated macro for shared_new_ref_data_with_space (function)
macro_rules! Depcrateshared_new_ref_data_with_space {
() => {
// Module: crate
// Provides: {"shared_new_ref_data_with_space"}
// Dependencies: {}
# [cfg (feature = "bincode")] fn shared_new_ref_data_with_space < T : serde :: Serialize , U : WritableAccount > (lamports : u64 , state : & T , space : usize , owner : & Pubkey ,) -> Result < RefCell < U > , bincode :: Error > { Ok (RefCell :: new (shared_new_data_with_space :: < T , U > (lamports , state , space , owner ,) ?)) }
};
}
