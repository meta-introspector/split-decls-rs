// Generated macro for gen_call_common_args (function)
macro_rules! Depcrate_machinst_islegen_call_common_args {
() => {
// Module: crate::machinst::isle
// Provides: {"gen_call_common_args"}
// Dependencies: {}
fn gen_call_common_args < M : ABIMachineSpec > (ctx : & mut Lower < '_ , M :: I > , call_site : & mut CallSite < M > , (inputs , off) : ValueSlice ,) { let num_args = call_site . num_args (ctx . sigs ()) ; assert_eq ! (inputs . len (& ctx . dfg () . value_lists) - off , num_args) ; let mut arg_regs = vec ! [] ; for i in 0 .. num_args { let input = inputs . get (off + i , & ctx . dfg () . value_lists) . unwrap () ; arg_regs . push (ctx . put_value_in_regs (input)) ; } for (i , arg_regs) in arg_regs . iter () . enumerate () { call_site . emit_copy_regs_to_buffer (ctx , i , * arg_regs) ; } for (i , arg_regs) in arg_regs . iter () . enumerate () { call_site . gen_arg (ctx , i , * arg_regs) ; } }
};
}
