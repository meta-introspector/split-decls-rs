// Generated macro for emit_vm_call (function)
macro_rules! Depcrate_isa_x64_loweremit_vm_call {
() => {
// Module: crate::isa::x64::lower
// Provides: {"emit_vm_call"}
// Dependencies: {}
fn emit_vm_call (ctx : & mut Lower < Inst > , flags : & Flags , triple : & Triple , libcall : LibCall , inputs : & [Reg] ,) -> CodegenResult < SmallVec < [Reg ; 1] > > { let extname = ExternalName :: LibCall (libcall) ; let dist = if flags . use_colocated_libcalls () { RelocDistance :: Near } else { RelocDistance :: Far } ; let call_conv = CallConv :: for_libcall (flags , CallConv :: triple_default (triple)) ; let sig = libcall . signature (call_conv , types :: I64) ; let caller_conv = ctx . abi () . call_conv (ctx . sigs ()) ; if ! ctx . sigs () . have_abi_sig_for_signature (& sig) { ctx . sigs_mut () . make_abi_sig_from_ir_signature :: < X64ABIMachineSpec > (sig . clone () , flags) ? ; } let mut abi = X64CallSite :: from_libcall (ctx . sigs () , & sig , & extname , dist , caller_conv , flags . clone ()) ; assert_eq ! (inputs . len () , abi . num_args (ctx . sigs ())) ; for (i , input) in inputs . iter () . enumerate () { abi . gen_arg (ctx , i , ValueRegs :: one (* input)) ; } let mut retval_insts : SmallInstVec < _ > = smallvec ! [] ; let mut outputs : SmallVec < [_ ; 1] > = smallvec ! [] ; for i in 0 .. ctx . sigs () . num_rets (ctx . sigs () . abi_sig_for_signature (& sig)) { let (retval_inst , retval_regs) = abi . gen_retval (ctx , i) ; retval_insts . extend (retval_inst . into_iter ()) ; outputs . push (retval_regs . only_reg () . unwrap ()) ; } abi . emit_call (ctx) ; for inst in retval_insts { ctx . emit (inst) ; } Ok (outputs) }
};
}
