// Generated macro for add_abi_diag_help (function)
macro_rules! Depcrate_check_checkadd_abi_diag_help {
() => {
// Module: crate::check::check
// Provides: {"add_abi_diag_help"}
// Dependencies: {}
fn add_abi_diag_help < T : EmissionGuarantee > (abi : ExternAbi , diag : & mut Diag < '_ , T >) { if let ExternAbi :: Cdecl { unwind } = abi { let c_abi = ExternAbi :: C { unwind } ; diag . help (format ! ("use `extern {c_abi}` instead" ,)) ; } else if let ExternAbi :: Stdcall { unwind } = abi { let c_abi = ExternAbi :: C { unwind } ; let system_abi = ExternAbi :: System { unwind } ; diag . help (format ! ("if you need `extern {abi}` on win32 and `extern {c_abi}` everywhere else, \
                use `extern {system_abi}`")) ; } }
};
}
