// Generated macro for impl_984 (impl)
macro_rules! Depcrateimpl_984 {
() => {
// Module: crate
// Provides: {"impl_984"}
// Dependencies: {}
impl ops :: Not for Side { type Output = Self ; fn not (self) -> Self { match self { Self :: Client => Self :: Server , Self :: Server => Self :: Client , } } }
};
}
