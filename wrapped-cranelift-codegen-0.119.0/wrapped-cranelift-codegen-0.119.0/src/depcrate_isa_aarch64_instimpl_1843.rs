// Generated macro for impl_1843 (impl)
macro_rules! Depcrate_isa_aarch64_instimpl_1843 {
() => {
// Module: crate::isa::aarch64::inst
// Provides: {"impl_1843"}
// Dependencies: {}
impl BitOp { # [doc = " Get the assembly mnemonic for this opcode."] pub fn op_str (& self) -> & 'static str { match self { BitOp :: RBit => "rbit" , BitOp :: Clz => "clz" , BitOp :: Cls => "cls" , BitOp :: Rev16 => "rev16" , BitOp :: Rev32 => "rev32" , BitOp :: Rev64 => "rev64" , } } }
};
}
