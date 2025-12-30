// Generated macro for encls (macro)
macro_rules! Depcrate_bits64_sgxencls {
() => {
// Module: crate::bits64::sgx
// Provides: {"encls"}
// Dependencies: {}
# [doc = " Execute an enclave system function of specified leaf number."] # [doc = ""] # [doc = " # Safety"] # [doc = "   * Function needs to be executed in ring 0."] macro_rules ! encls { ($ rax : expr , $ rbx : expr) => { $ crate :: bits64 :: sgx :: encls2 ($ rax as u64 , $ rbx as u64) } ; ($ rax : expr , $ rbx : expr , $ rcx : expr) => { $ crate :: bits64 :: sgx :: encls3 ($ rax as u64 , $ rbx as u64 , $ rcx as u64) } ; ($ rax : expr , $ rbx : expr , $ rcx : expr , $ rdx : expr) => { $ crate :: bits64 :: sgx :: encls4 ($ rax as u64 , $ rbx as u64 , $ rcx as u64 , $ rdx as u64) } ; }
};
}
