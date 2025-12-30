// Generated macro for impl_1307 (impl)
macro_rules! Depcrate_isa_x64_inst_argsimpl_1307 {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"impl_1307"}
// Dependencies: {}
impl From < RegMem > for RegMemImm { fn from (rm : RegMem) -> RegMemImm { match rm { RegMem :: Reg { reg } => RegMemImm :: Reg { reg } , RegMem :: Mem { addr } => RegMemImm :: Mem { addr } , } } }
};
}
