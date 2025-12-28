macro_rules! SanitizerOptions {
    () => {
        # [doc = " LLVMRustSanitizerOptions"] # [repr (C)] pub (crate) struct SanitizerOptions { pub sanitize_address : bool , pub sanitize_address_recover : bool , pub sanitize_cfi : bool , pub sanitize_dataflow : bool , pub sanitize_dataflow_abilist : * const * const c_char , pub sanitize_dataflow_abilist_len : size_t , pub sanitize_kcfi : bool , pub sanitize_memory : bool , pub sanitize_memory_recover : bool , pub sanitize_memory_track_origins : c_int , pub sanitize_thread : bool , pub sanitize_hwaddress : bool , pub sanitize_hwaddress_recover : bool , pub sanitize_kernel_address : bool , pub sanitize_kernel_address_recover : bool , }
    };
}

SanitizerOptions!()