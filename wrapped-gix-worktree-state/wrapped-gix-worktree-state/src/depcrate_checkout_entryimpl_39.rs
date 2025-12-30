// Generated macro for impl_39 (impl)
macro_rules! Depcrate_checkout_entryimpl_39 {
() => {
// Module: crate::checkout::entry
// Provides: {"impl_39"}
// Dependencies: {}
impl Outcome < '_ > { # [doc = " Return ourselves as (in-memory) bytes if possible."] pub fn as_bytes (& self) -> Option < usize > { match self { Outcome :: Written { bytes } => Some (* bytes) , Outcome :: Delayed { .. } => None , } } }
};
}
