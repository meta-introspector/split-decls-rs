// Generated macro for instructions (module)
macro_rules! Depcrate_sysvarinstructions {
() => {
// Module: crate::sysvar
// Provides: {"instructions"}
// Dependencies: {}
pub mod instructions { # [deprecated (since = "2.2.0" , note = "Use solana-instruction crate instead")] pub use solana_instruction :: { BorrowedAccountMeta , BorrowedInstruction } ; # [cfg (not (target_os = "solana"))] # [deprecated (since = "2.2.0" , note = "Use solana-instructions-sysvar crate instead")] pub use solana_instructions_sysvar :: construct_instructions_data ; # [cfg (all (not (target_os = "solana") , feature = "dev-context-only-utils"))] # [deprecated (since = "2.2.0" , note = "Use solana-instructions-sysvar crate instead")] pub use solana_instructions_sysvar :: serialize_instructions ; # [cfg (feature = "dev-context-only-utils")] # [deprecated (since = "2.2.0" , note = "Use solana-instructions-sysvar crate instead")] pub use solana_instructions_sysvar :: { deserialize_instruction , load_instruction_at } ; # [deprecated (since = "2.2.0" , note = "Use solana-instructions-sysvar crate instead")] # [allow (deprecated)] pub use solana_instructions_sysvar :: { get_instruction_relative , load_current_index_checked , load_instruction_at_checked , Instructions , } ; # [deprecated (since = "2.2.0" , note = "Use solana-sdk-ids crate instead")] pub use solana_sdk_ids :: sysvar :: instructions :: { check_id , id , ID } ; }
};
}
