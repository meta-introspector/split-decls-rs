// Generated macro for impl_49 (impl)
macro_rules! Depcrateimpl_49 {
() => {
// Module: crate
// Provides: {"impl_49"}
// Dependencies: {}
impl AsRef < [u8] > for Minor { # [inline] fn as_ref (& self) -> & [u8] { match self { Self :: More => & [] , Self :: This (..) => & [] , Self :: Next1 (x) => x . as_ref () , Self :: Next2 (x) => x . as_ref () , Self :: Next4 (x) => x . as_ref () , Self :: Next8 (x) => x . as_ref () , } } }
};
}
