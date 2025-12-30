// Generated macro for impl_50 (impl)
macro_rules! Depcrateimpl_50 {
() => {
// Module: crate
// Provides: {"impl_50"}
// Dependencies: {}
impl AsMut < [u8] > for Minor { # [inline] fn as_mut (& mut self) -> & mut [u8] { match self { Self :: More => & mut [] , Self :: This (..) => & mut [] , Self :: Next1 (x) => x . as_mut () , Self :: Next2 (x) => x . as_mut () , Self :: Next4 (x) => x . as_mut () , Self :: Next8 (x) => x . as_mut () , } } }
};
}
