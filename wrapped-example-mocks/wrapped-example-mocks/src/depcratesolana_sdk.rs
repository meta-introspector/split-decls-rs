// Generated macro for solana_sdk (module)
macro_rules! Depcratesolana_sdk {
() => {
// Module: crate
// Provides: {"solana_sdk"}
// Dependencies: {}
# [doc = " Re-exports and mocks of solana-program modules that mirror those from"] # [doc = " solana-program."] # [doc = ""] # [doc = " This lets examples in solana-program appear to be written as client"] # [doc = " programs."] pub mod solana_sdk { pub use { crate :: { solana_account :: { self as account , state_traits as account_utils } , solana_signer :: { self as signer , signers } , } , solana_clock :: Clock , solana_hash as hash , solana_instruction as instruction , solana_keccak_hasher as keccak , solana_message as message , solana_nonce as nonce , solana_pubkey :: { self as pubkey , Pubkey } , solana_sdk_ids :: { system_program , sysvar :: { self , clock } , } , solana_system_interface :: instruction as system_instruction , } ; pub mod signature { pub use crate :: { solana_keypair :: Keypair , solana_signature :: Signature , solana_signer :: Signer , } ; } pub mod transaction { pub use crate :: solana_transaction :: { versioned :: VersionedTransaction , Transaction } ; } pub mod address_lookup_table { pub use { solana_address_lookup_table_interface :: { error , instruction , program , state } , solana_message :: AddressLookupTableAccount , } ; } }
};
}
