// Generated macro for BytesMode (enum)
macro_rules! Depcrate_configBytesMode {
() => {
// Module: crate::config
// Provides: {"BytesMode"}
// Dependencies: {}
# [doc = " When to encode `[u8]` as `bytes` rather than a sequence"] # [doc = " of integers. Serde without `serde_bytes` has trouble"] # [doc = " using `bytes`, and this is hack to force it. It may"] # [doc = " break some data types."] # [non_exhaustive] # [derive (Debug , Copy , Clone , Default , PartialEq , Eq)] pub enum BytesMode { # [doc = " Use bytes only when Serde requires it"] # [doc = " (typically only when `serde_bytes` is used)"] # [default] Normal , # [doc = " Use bytes for slices, `Vec`, and a few other types that"] # [doc = " use `Iterator` in Serde."] # [doc = ""] # [doc = " This may break some implementations of `Deserialize`."] # [doc = ""] # [doc = " This does not include fixed-length arrays."] ForceIterables , # [doc = " Use bytes for everything that looks like a container of `u8`."] # [doc = " This breaks some implementations of `Deserialize`."] ForceAll , }
};
}
