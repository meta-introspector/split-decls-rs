// Generated macro for VRegAllocator (struct)
macro_rules! Depcrate_machinst_vcodeVRegAllocator {
() => {
// Module: crate::machinst::vcode
// Provides: {"VRegAllocator"}
// Dependencies: {}
# [doc = " This structure manages VReg allocation during the lifetime of the VCodeBuilder."] pub struct VRegAllocator < I > { # [doc = " VReg IR-level types."] vreg_types : Vec < Type > , # [doc = " VReg aliases. When the final VCode is built we rewrite all"] # [doc = " uses of the keys in this table to their replacement values."] # [doc = ""] # [doc = " We use these aliases to rename an instruction's expected"] # [doc = " result vregs to the returned vregs from lowering, which are"] # [doc = " usually freshly-allocated temps."] vreg_aliases : FxHashMap < regalloc2 :: VReg , regalloc2 :: VReg > , # [doc = " A deferred error, to be bubbled up to the top level of the"] # [doc = " lowering algorithm. We take this approach because we cannot"] # [doc = " currently propagate a `Result` upward through ISLE code (the"] # [doc = " lowering rules) or some ABI code."] deferred_error : Option < CodegenError > , # [doc = " Facts on VRegs, for proof-carrying code."] facts : Vec < Option < Fact > > , # [doc = " The type of instruction that this allocator makes registers for."] _inst : core :: marker :: PhantomData < I > , }
};
}
