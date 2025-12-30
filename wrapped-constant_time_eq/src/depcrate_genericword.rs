// Generated macro for Word (type)
macro_rules! Depcrate_genericWord {
() => {
// Module: crate::generic
// Provides: {"Word"}
// Dependencies: {}
# [doc = " The natural word type for this architecture. All bit patterns must be valid for this type."] # [cfg (all (any (target_arch = "riscv64" , target_arch = "riscv32") , not (target_feature = "unaligned-scalar-mem")))] pub (crate) type Word = u8 ;
};
}
