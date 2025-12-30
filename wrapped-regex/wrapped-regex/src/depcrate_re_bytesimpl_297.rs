// Generated macro for impl_297 (impl)
macro_rules! Depcrate_re_bytesimpl_297 {
() => {
// Module: crate::re_bytes
// Provides: {"impl_297"}
// Dependencies: {}
# [doc = " A constructor for Regex from an Exec."] # [doc = ""] # [doc = " This is hidden because Exec isn't actually part of the public API."] # [doc (hidden)] impl From < Exec > for Regex { fn from (exec : Exec) -> Regex { Regex (exec) } }
};
}
