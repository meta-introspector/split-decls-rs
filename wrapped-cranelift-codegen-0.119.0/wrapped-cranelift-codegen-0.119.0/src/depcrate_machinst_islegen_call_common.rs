// Generated macro for gen_call_common (function)
macro_rules! Depcrate_machinst_islegen_call_common {
() => {
// Module: crate::machinst::isle
// Provides: {"gen_call_common"}
// Dependencies: {}
pub fn gen_call_common < M : ABIMachineSpec > (ctx : & mut Lower < '_ , M :: I > , num_rets : usize , mut caller : CallSite < M > , args : ValueSlice ,) -> InstOutput { gen_call_common_args (ctx , & mut caller , args) ; let mut outputs = InstOutput :: new () ; let mut retval_insts = crate :: machinst :: abi :: SmallInstVec :: new () ; let sigdata_num_rets = caller . num_rets (ctx . sigs ()) ; debug_assert ! (num_rets <= sigdata_num_rets) ; for i in (sigdata_num_rets - num_rets) .. sigdata_num_rets { let (retval_inst , retval_regs) = caller . gen_retval (ctx , i) ; retval_insts . extend (retval_inst . into_iter ()) ; outputs . push (retval_regs) ; } caller . emit_call (ctx) ; for inst in retval_insts { ctx . emit (inst) ; } outputs }
};
}
