// Generated macro for additional_notes (function)
macro_rules! Depcrate_pointers_in_nomem_asm_blockadditional_notes {
() => {
// Module: crate::pointers_in_nomem_asm_block
// Provides: {"additional_notes"}
// Dependencies: {}
fn additional_notes (diag : & mut rustc_errors :: Diag < '_ , () >) { diag . note ("`nomem` means that no memory write or read happens inside the asm! block") ; diag . note ("if this is intentional and no pointers are read or written to, consider allowing the lint") ; }
};
}
