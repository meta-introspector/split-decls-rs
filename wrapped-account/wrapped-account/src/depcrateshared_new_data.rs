// Generated macro for shared_new_data (function)
macro_rules! Depcrateshared_new_data {
() => {
// Module: crate
// Provides: {"shared_new_data"}
// Dependencies: {}
# [cfg (feature = "bincode")] fn shared_new_data < T : serde :: Serialize , U : WritableAccount > (lamports : u64 , state : & T , owner : & Pubkey ,) -> Result < U , bincode :: Error > { let data = bincode :: serialize (state) ? ; Ok (U :: create (lamports , data , * owner , bool :: default () , Epoch :: default () ,)) }
};
}
