// Generated macro for enclu (macro)
macro_rules! Depcrate_bits64_sgxenclu {
() => {
// Module: crate::bits64::sgx
// Provides: {"enclu"}
// Dependencies: {}
# [doc = " Execute an enclave user function of specified leaf number."] # [doc = ""] # [doc = " # Safety"] # [doc = "   * Function needs to be executed in ring 3."] macro_rules ! enclu { ($ rax : expr , $ rbx : expr , $ rcx : expr) => { $ crate :: bits64 :: sgx :: enclu3 ($ rax as u64 , $ rbx as u64 , $ rcx as u64) } ; ($ rax : expr , $ rbx : expr , $ rcx : expr , $ rdx : expr) => { $ crate :: bits64 :: sgx :: enclu4 ($ rax as u64 , $ rbx as u64 , $ rcx as u64 , $ rdx as u64) } ; }
};
}
