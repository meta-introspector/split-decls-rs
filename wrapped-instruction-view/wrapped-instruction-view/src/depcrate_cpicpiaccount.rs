// Generated macro for CpiAccount (struct)
macro_rules! Depcrate_cpiCpiAccount {
() => {
// Module: crate::cpi
// Provides: {"CpiAccount"}
// Dependencies: {}
# [doc = " An account for CPI invocations."] # [doc = ""] # [doc = " This struct contains the same information as an [`AccountView`], but has"] # [doc = " the memory layout as expected by `sol_invoke_signed_c` syscall."] # [repr (C)] # [derive (Clone , Copy , Debug)] pub struct CpiAccount < 'a > { # [doc = " Address of the account."] address : * const Address , # [doc = " Number of lamports owned by this account."] lamports : * const u64 , # [doc = " Length of data in bytes."] data_len : u64 , # [doc = " On-chain data within this account."] data : * const u8 , # [doc = " Program that owns this account."] owner : * const Address , # [doc = " The epoch at which this account will next owe rent."] rent_epoch : u64 , # [doc = " Transaction was signed by this account's key?"] is_signer : bool , # [doc = " Is the account writable?"] is_writable : bool , # [doc = " This account's data contains a loaded program (and is now read-only)."] executable : bool , # [doc = " The pointers to the `AccountView` data are only valid for as long as the"] # [doc = " `&'a AccountView` lives. Instead of holding a reference to the actual `AccountView`,"] # [doc = " which would increase the size of the type, we claim to hold a reference without"] # [doc = " actually holding one using a `PhantomData<&'a AccountView>`."] _account_view : PhantomData < & 'a AccountView > , }
};
}
