// Generated macro for impl_281 (impl)
macro_rules! Depcrateimpl_281 {
() => {
// Module: crate
// Provides: {"impl_281"}
// Dependencies: {}
# [cfg (not (feature = "rustc-dep-of-std"))] impl DataFormat { pub fn from_window_bits (window_bits : i32) -> DataFormat { if window_bits > 0 { DataFormat :: Zlib } else { DataFormat :: Raw } } pub fn to_window_bits (self) -> i32 { match self { DataFormat :: Zlib | DataFormat :: ZLibIgnoreChecksum => shared :: MZ_DEFAULT_WINDOW_BITS , DataFormat :: Raw => - shared :: MZ_DEFAULT_WINDOW_BITS , } } }
};
}
