// Generated macro for impl_1414 (impl)
macro_rules! Depcrate_isa_x64_inst_externalimpl_1414 {
() => {
// Module: crate::isa::x64::inst::external
// Provides: {"impl_1414"}
// Dependencies: {}
impl Into < asm :: Amode < Gpr > > for SyntheticAmode { fn into (self) -> asm :: Amode < Gpr > { match self { SyntheticAmode :: Real (amode) => amode . into () , SyntheticAmode :: IncomingArg { offset } => asm :: Amode :: ImmReg { base : Gpr :: unwrap_new (regs :: rbp ()) , simm32 : asm :: AmodeOffsetPlusKnownOffset { simm32 : (- i32 :: try_from (offset) . unwrap ()) . into () , offset : Some (offsets :: KEY_INCOMING_ARG) , } , trap : None , } , SyntheticAmode :: SlotOffset { simm32 } => asm :: Amode :: ImmReg { base : Gpr :: unwrap_new (regs :: rbp ()) , simm32 : asm :: AmodeOffsetPlusKnownOffset { simm32 : simm32 . into () , offset : Some (offsets :: KEY_SLOT_OFFSET) , } , trap : None , } , SyntheticAmode :: ConstantOffset (vcode_constant) => asm :: Amode :: RipRelative { target : asm :: DeferredTarget :: Constant (asm :: Constant (vcode_constant . as_u32 ())) , } , } } }
};
}
