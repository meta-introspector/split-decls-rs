// Generated macro for non_bpf_modules (module)
macro_rules! Depcratenon_bpf_modules {
() => {
// Module: crate
// Provides: {"non_bpf_modules"}
// Dependencies: {}
# [cfg (not (target_os = "solana"))] # [path = ""] mod non_bpf_modules { mod account_keys ; mod address_loader ; mod sanitized ; mod versions ; pub use { account_keys :: * , address_loader :: * , sanitized :: * , versions :: * } ; }
};
}
