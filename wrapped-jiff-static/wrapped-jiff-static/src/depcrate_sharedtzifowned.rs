// Generated macro for TzifOwned (type)
macro_rules! Depcrate_sharedTzifOwned {
() => {
// Module: crate::shared
// Provides: {"TzifOwned"}
// Dependencies: {}
# [doc = " An alias for TZif data whose backing storage has a `'static` lifetime."] # [doc = " An alias for TZif data whose backing storage is on the heap."] pub type TzifOwned = Tzif < alloc :: string :: String , self :: util :: array_str :: Abbreviation , alloc :: vec :: Vec < TzifLocalTimeType > , alloc :: vec :: Vec < i64 > , alloc :: vec :: Vec < TzifDateTime > , alloc :: vec :: Vec < TzifDateTime > , alloc :: vec :: Vec < TzifTransitionInfo > , > ;
};
}
