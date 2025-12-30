// Generated macro for get_system_account_kind (function)
macro_rules! Depcrateget_system_account_kind {
() => {
// Module: crate
// Provides: {"get_system_account_kind"}
// Dependencies: {}
pub fn get_system_account_kind (account : & AccountSharedData) -> Option < SystemAccountKind > { if system_program :: check_id (account . owner ()) { if account . data () . is_empty () { Some (SystemAccountKind :: System) } else if account . data () . len () == State :: size () { let nonce_versions : Versions = account . state () . ok () ? ; match nonce_versions . state () { State :: Uninitialized => None , State :: Initialized (_) => Some (SystemAccountKind :: Nonce) , } } else { None } } else { None } }
};
}
