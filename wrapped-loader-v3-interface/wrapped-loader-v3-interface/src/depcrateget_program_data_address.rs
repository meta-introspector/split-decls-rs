// Generated macro for get_program_data_address (function)
macro_rules! Depcrateget_program_data_address {
() => {
// Module: crate
// Provides: {"get_program_data_address"}
// Dependencies: {}
# [doc = " Returns the program data address for a program ID"] pub fn get_program_data_address (program_address : & Pubkey) -> Pubkey { Pubkey :: find_program_address (& [program_address . as_ref ()] , & solana_sdk_ids :: bpf_loader_upgradeable :: id () ,) . 0 }
};
}
