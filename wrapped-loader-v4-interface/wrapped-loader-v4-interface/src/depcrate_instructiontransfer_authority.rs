// Generated macro for transfer_authority (function)
macro_rules! Depcrate_instructiontransfer_authority {
() => {
// Module: crate::instruction
// Provides: {"transfer_authority"}
// Dependencies: {}
# [doc = " Returns the instructions required to transfer authority over a program."] # [cfg (feature = "bincode")] pub fn transfer_authority (program_address : & Pubkey , authority : & Pubkey , new_authority : & Pubkey ,) -> Instruction { let accounts = vec ! [AccountMeta :: new (* program_address , false) , AccountMeta :: new_readonly (* authority , true) , AccountMeta :: new_readonly (* new_authority , true) ,] ; Instruction :: new_with_bincode (id () , & LoaderV4Instruction :: TransferAuthority , accounts) }
};
}
