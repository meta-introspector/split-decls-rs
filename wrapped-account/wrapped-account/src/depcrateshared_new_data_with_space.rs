// Generated macro for shared_new_data_with_space (function)
macro_rules! Depcrateshared_new_data_with_space {
() => {
// Module: crate
// Provides: {"shared_new_data_with_space"}
// Dependencies: {}
# [cfg (feature = "bincode")] fn shared_new_data_with_space < T : serde :: Serialize , U : WritableAccount > (lamports : u64 , state : & T , space : usize , owner : & Pubkey ,) -> Result < U , bincode :: Error > { let mut account = shared_new :: < U > (lamports , space , owner) ; shared_serialize_data (& mut account , state) ? ; Ok (account) }
};
}
