// Generated macro for BorrowedInstruction (struct)
macro_rules! DepcrateBorrowedInstruction {
() => {
// Module: crate
// Provides: {"BorrowedInstruction"}
// Dependencies: {}
# [doc = " Borrowed version of `Instruction`."] # [doc = ""] # [doc = " This struct is used by the runtime when constructing the instructions sysvar. It is not"] # [doc = " useful to Solana programs."] # [cfg (feature = "std")] pub struct BorrowedInstruction < 'a > { pub program_id : & 'a Pubkey , pub accounts : Vec < BorrowedAccountMeta < 'a > > , pub data : & 'a [u8] , }
};
}
