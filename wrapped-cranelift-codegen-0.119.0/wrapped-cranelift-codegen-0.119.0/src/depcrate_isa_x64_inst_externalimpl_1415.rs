// Generated macro for impl_1415 (impl)
macro_rules! Depcrate_isa_x64_inst_externalimpl_1415 {
() => {
// Module: crate::isa::x64::inst::external
// Provides: {"impl_1415"}
// Dependencies: {}
impl Into < asm :: Amode < Gpr > > for Amode { fn into (self) -> asm :: Amode < Gpr > { match self { Amode :: ImmReg { simm32 , base , flags , } => asm :: Amode :: ImmReg { simm32 : asm :: AmodeOffsetPlusKnownOffset { simm32 : simm32 . into () , offset : None , } , base : Gpr :: unwrap_new (base) , trap : flags . trap_code () . map (Into :: into) , } , Amode :: ImmRegRegShift { simm32 , base , index , shift , flags , } => asm :: Amode :: ImmRegRegShift { base , index : asm :: NonRspGpr :: new (index) , scale : asm :: Scale :: new (shift) , simm32 : simm32 . into () , trap : flags . trap_code () . map (Into :: into) , } , Amode :: RipRelative { target } => asm :: Amode :: RipRelative { target : asm :: DeferredTarget :: Label (asm :: Label (target . as_u32 ())) , } , } } }
};
}
