// Generated macro for Bytes (struct)
macro_rules! Depcrate_iterBytes {
() => {
// Module: crate::iter
// Provides: {"Bytes"}
// Dependencies: {}
# [allow (missing_docs)] pub struct Bytes < 'a > { start : * const u8 , end : * const u8 , # [doc = " INVARIANT: start <= cursor && cursor <= end"] cursor : * const u8 , phantom : core :: marker :: PhantomData < & 'a () > , }
};
}
