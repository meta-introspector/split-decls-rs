// Generated macro for gen_stack_limit (function)
macro_rules! Depcrate_machinst_abigen_stack_limit {
() => {
// Module: crate::machinst::abi
// Provides: {"gen_stack_limit"}
// Dependencies: {}
# [doc = " Generates the instructions necessary for the `gv` to be materialized into a"] # [doc = " register."] # [doc = ""] # [doc = " This function will return a register that will contain the result of"] # [doc = " evaluating `gv`. It will also return any instructions necessary to calculate"] # [doc = " the value of the register."] # [doc = ""] # [doc = " Note that global values are typically lowered to instructions via the"] # [doc = " standard legalization pass. Unfortunately though prologue generation happens"] # [doc = " so late in the pipeline that we can't use these legalization passes to"] # [doc = " generate the instructions for `gv`. As a result we duplicate some lowering"] # [doc = " of `gv` here and support only some global values. This is similar to what"] # [doc = " the x86 backend does for now, and hopefully this can be somewhat cleaned up"] # [doc = " in the future too!"] # [doc = ""] # [doc = " Also note that this function will make use of `writable_spilltmp_reg()` as a"] # [doc = " temporary register to store values in if necessary. Currently after we write"] # [doc = " to this register there's guaranteed to be no spilled values between where"] # [doc = " it's used, because we're not participating in register allocation anyway!"] fn gen_stack_limit < M : ABIMachineSpec > (f : & ir :: Function , sigs : & SigSet , sig : Sig , gv : ir :: GlobalValue ,) -> (Reg , SmallInstVec < M :: I >) { let mut insts = smallvec ! [] ; let reg = generate_gv :: < M > (f , sigs , sig , gv , & mut insts) ; return (reg , insts) ; }
};
}
