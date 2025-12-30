// Generated macro for lookup (function)
macro_rules! Depcrate_isalookup {
() => {
// Module: crate::isa
// Provides: {"lookup"}
// Dependencies: {}
# [doc = " Look for an ISA for the given `triple`."] # [doc = " Return a builder that can create a corresponding `TargetIsa`."] pub fn lookup (triple : Triple) -> Result < Builder , LookupError > { match triple . architecture { Architecture :: X86_64 => { isa_builder ! (x64 , (feature = "x86") , triple) } Architecture :: Aarch64 { .. } => isa_builder ! (aarch64 , (feature = "arm64") , triple) , Architecture :: S390x { .. } => isa_builder ! (s390x , (feature = "s390x") , triple) , Architecture :: Riscv64 { .. } => isa_builder ! (riscv64 , (feature = "riscv64") , triple) , Architecture :: Pulley32 | Architecture :: Pulley32be => { isa_builder ! (pulley32 , (feature = "pulley") , triple) } Architecture :: Pulley64 | Architecture :: Pulley64be => { isa_builder ! (pulley64 , (feature = "pulley") , triple) } _ => Err (LookupError :: Unsupported) , } }
};
}
