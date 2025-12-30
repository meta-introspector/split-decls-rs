// Generated macro for AccountInfo (struct)
macro_rules! DepcrateAccountInfo {
() => {
// Module: crate
// Provides: {"AccountInfo"}
// Dependencies: {}
# [doc = " Account information"] # [derive (Clone)] # [repr (C)] pub struct AccountInfo < 'a > { # [doc = " Address of the account"] pub key : & 'a Address , # [doc = " The lamports in the account.  Modifiable by programs."] pub lamports : Rc < RefCell < & 'a mut u64 > > , # [doc = " The data held in this account.  Modifiable by programs."] pub data : Rc < RefCell < & 'a mut [u8] > > , # [doc = " Program that owns this account"] pub owner : & 'a Address , # [doc = " Formerly, the epoch at which this account will next owe rent. A field"] # [doc = " must remain because the runtime depends on the exact layout of this"] # [doc = " struct."] # [deprecated (since = "3.0.0" , note = "Do not use this field, it will not exist in ABIv2")] pub _unused : u64 , # [doc = " Was the transaction signed by this account's public key?"] pub is_signer : bool , # [doc = " Is the account writable?"] pub is_writable : bool , # [doc = " This account's data contains a loaded program (and is now read-only)"] pub executable : bool , }
};
}
