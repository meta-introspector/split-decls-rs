// Generated macro for test (module)
macro_rules! Depcrate_arbitrary__core_non_zerotest {
() => {
// Module: crate::arbitrary::_core::non_zero
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { no_panic_test ! (u8 => core :: num :: NonZeroU8 , u16 => core :: num :: NonZeroU16 , u32 => core :: num :: NonZeroU32 , u64 => core :: num :: NonZeroU64 , usize => core :: num :: NonZeroUsize , i8 => core :: num :: NonZeroI8 , i16 => core :: num :: NonZeroI16 , i32 => core :: num :: NonZeroI32 , i64 => core :: num :: NonZeroI64 , isize => core :: num :: NonZeroIsize) ; # [cfg (not (target_arch = "wasm32"))] no_panic_test ! (u128 => core :: num :: NonZeroU128 , i128 => core :: num :: NonZeroI128) ; }
};
}
