// Generated macro for create_is_signer_account_infos (function)
macro_rules! Depcratecreate_is_signer_account_infos {
() => {
// Module: crate
// Provides: {"create_is_signer_account_infos"}
// Dependencies: {}
# [doc = " Create `AccountInfo`s"] pub fn create_is_signer_account_infos < 'a > (accounts : & 'a mut [(& 'a Pubkey , bool , & 'a mut Account)] ,) -> Vec < AccountInfo < 'a > > { accounts . iter_mut () . map (| (key , is_signer , account) | { AccountInfo :: new (key , * is_signer , false , & mut account . lamports , & mut account . data , & account . owner , account . executable ,) }) . collect () }
};
}
