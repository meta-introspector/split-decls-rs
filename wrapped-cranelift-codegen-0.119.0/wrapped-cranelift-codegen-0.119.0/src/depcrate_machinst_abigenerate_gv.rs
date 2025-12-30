// Generated macro for generate_gv (function)
macro_rules! Depcrate_machinst_abigenerate_gv {
() => {
// Module: crate::machinst::abi
// Provides: {"generate_gv"}
// Dependencies: {}
fn generate_gv < M : ABIMachineSpec > (f : & ir :: Function , sigs : & SigSet , sig : Sig , gv : ir :: GlobalValue , insts : & mut SmallInstVec < M :: I > ,) -> Reg { match f . global_values [gv] { ir :: GlobalValueData :: VMContext => { get_special_purpose_param_register (f , sigs , sig , ir :: ArgumentPurpose :: VMContext) . expect ("no vmcontext parameter found") } ir :: GlobalValueData :: Load { base , offset , global_type : _ , flags : _ , } => { let base = generate_gv :: < M > (f , sigs , sig , base , insts) ; let into_reg = Writable :: from_reg (M :: get_stacklimit_reg (f . stencil . signature . call_conv)) ; insts . push (M :: gen_load_base_offset (into_reg , base , offset . into () , M :: word_type () ,)) ; return into_reg . to_reg () ; } ref other => panic ! ("global value for stack limit not supported: {other}") , } }
};
}
