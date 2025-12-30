// Generated macro for create_account_with_fields (function)
macro_rules! Depcratecreate_account_with_fields {
() => {
// Module: crate
// Provides: {"create_account_with_fields"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub fn create_account_with_fields < S : SysvarSerialize > (sysvar : & S , (lamports , rent_epoch) : InheritableAccountFields ,) -> Account { let data_len = S :: size_of () . max (bincode :: serialized_size (sysvar) . unwrap () as usize) ; let mut account = Account :: new (lamports , data_len , & solana_sdk_ids :: sysvar :: id ()) ; to_account :: < S , Account > (sysvar , & mut account) . unwrap () ; account . rent_epoch = rent_epoch ; account }
};
}
