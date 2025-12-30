// Generated macro for create_account (function)
macro_rules! Depcrate_statecreate_account {
() => {
// Module: crate::state
// Provides: {"create_account"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub fn create_account (feature : & Feature , lamports : u64) -> AccountSharedData { let data_len = Feature :: size_of () . max (bincode :: serialized_size (feature) . unwrap () as usize) ; let mut account = AccountSharedData :: new (lamports , data_len , & id ()) ; to_account (feature , & mut account) . unwrap () ; account }
};
}
