// Generated macro for tests (module)
macro_rules! Depcrate_isa_x64_inst_externaltests {
() => {
// Module: crate::isa::x64::inst::external
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: asm :: { AsReg , Size } ; use super :: PairedGpr ; use crate :: isa :: x64 :: args :: { FromWritableReg , Gpr , WritableGpr , WritableXmm , Xmm } ; use crate :: isa :: x64 :: inst :: external :: PairedXmm ; use crate :: { Reg , Writable } ; use regalloc2 :: { RegClass , VReg } ; # [test] fn pretty_print_registers () { let v200 : Reg = VReg :: new (200 , RegClass :: Int) . into () ; let gpr200 = Gpr :: new (v200) . unwrap () ; assert_eq ! (gpr200 . to_string (Some (Size :: Quadword)) , "%v200") ; let v300 : Reg = VReg :: new (300 , RegClass :: Int) . into () ; let wgpr300 = WritableGpr :: from_writable_reg (Writable :: from_reg (v300) . into ()) . unwrap () ; let pair = PairedGpr { read : gpr200 , write : wgpr300 , } ; assert_eq ! (pair . to_string (Some (Size :: Quadword)) , "(%v300 <- %v200)") ; let v400 : Reg = VReg :: new (400 , RegClass :: Float) . into () ; let xmm400 = Xmm :: new (v400) . unwrap () ; assert_eq ! (xmm400 . to_string (None) , "%v400") ; let v500 : Reg = VReg :: new (500 , RegClass :: Float) . into () ; let wxmm500 = WritableXmm :: from_writable_reg (Writable :: from_reg (v500) . into ()) . unwrap () ; let pair = PairedXmm { read : xmm400 , write : wxmm500 , } ; assert_eq ! (pair . to_string (None) , "(%v500 <- %v400)") ; } }
};
}
