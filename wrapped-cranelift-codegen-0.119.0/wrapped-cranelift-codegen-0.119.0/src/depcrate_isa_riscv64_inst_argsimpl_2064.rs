// Generated macro for impl_2064 (impl)
macro_rules! Depcrate_isa_riscv64_inst_argsimpl_2064 {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"impl_2064"}
// Dependencies: {}
# [allow (dead_code)] impl FClassResult { pub (crate) const fn bit (self) -> u32 { match self { FClassResult :: NegInfinite => 1 << 0 , FClassResult :: NegNormal => 1 << 1 , FClassResult :: NegSubNormal => 1 << 2 , FClassResult :: NegZero => 1 << 3 , FClassResult :: PosZero => 1 << 4 , FClassResult :: PosSubNormal => 1 << 5 , FClassResult :: PosNormal => 1 << 6 , FClassResult :: PosInfinite => 1 << 7 , FClassResult :: SNaN => 1 << 8 , FClassResult :: QNaN => 1 << 9 , } } # [inline] pub (crate) const fn is_nan_bits () -> u32 { Self :: SNaN . bit () | Self :: QNaN . bit () } # [inline] pub (crate) fn is_zero_bits () -> u32 { Self :: NegZero . bit () | Self :: PosZero . bit () } # [inline] pub (crate) fn is_infinite_bits () -> u32 { Self :: PosInfinite . bit () | Self :: NegInfinite . bit () } }
};
}
