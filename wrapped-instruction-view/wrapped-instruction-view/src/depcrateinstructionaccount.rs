// Generated macro for InstructionAccount (struct)
macro_rules! DepcrateInstructionAccount {
() => {
// Module: crate
// Provides: {"InstructionAccount"}
// Dependencies: {}
# [doc = " Describes an account during instruction execution."] # [doc = ""] # [doc = " When constructing an [`InstructionView`], a list of all accounts that may be"] # [doc = " signer, read or written during the execution of that instruction must be supplied."] # [doc = " Any account that may be mutated by the program during execution, either its"] # [doc = " data or metadata such as held lamports, must be writable."] # [doc = ""] # [doc = " Note that because the Solana runtime schedules parallel transaction"] # [doc = " execution around which accounts are writable, care should be taken that only"] # [doc = " accounts which actually may be mutated are specified as writable."] # [repr (C)] # [derive (Debug , Clone)] pub struct InstructionAccount < 'a > { # [doc = " Address of the account."] pub address : & 'a Address , # [doc = " Indicates whether the account is writable or not."] pub is_writable : bool , # [doc = " Indicates whether the account signed the instruction or not."] pub is_signer : bool , }
};
}
