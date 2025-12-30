// Generated macro for impl_1319 (impl)
macro_rules! Depcrate_isa_x64_inst_argsimpl_1319 {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"impl_1319"}
// Dependencies: {}
impl fmt :: Debug for AluRmiROpcode { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { let name = match self { AluRmiROpcode :: Add => "add" , AluRmiROpcode :: Adc => "adc" , AluRmiROpcode :: Sub => "sub" , AluRmiROpcode :: Sbb => "sbb" , AluRmiROpcode :: And => "and" , AluRmiROpcode :: Or => "or" , AluRmiROpcode :: Xor => "xor" , } ; write ! (fmt , "{name}") } }
};
}
