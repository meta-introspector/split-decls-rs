// Generated macro for BorrowedAccountMeta (struct)
macro_rules! DepcrateBorrowedAccountMeta {
() => {
// Module: crate
// Provides: {"BorrowedAccountMeta"}
// Dependencies: {}
# [doc = " Borrowed version of `AccountMeta`."] # [doc = ""] # [doc = " This struct is used by the runtime when constructing the instructions sysvar. It is not"] # [doc = " useful to Solana programs."] pub struct BorrowedAccountMeta < 'a > { pub pubkey : & 'a Pubkey , pub is_signer : bool , pub is_writable : bool , }
};
}
