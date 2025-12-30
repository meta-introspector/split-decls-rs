// Generated macro for WritebackOnDropNullable (struct)
macro_rules! Depcrate___macros_writebackWritebackOnDropNullable {
() => {
// Module: crate::__macros::writeback
// Provides: {"WritebackOnDropNullable"}
// Dependencies: {}
# [doc = " Mostly the same as `WritebackOnDrop`, except that the old value is"] # [doc = " nullable."] # [derive (Debug)] pub struct WritebackOnDropNullable < T : Message > { ptr : NonNull < * mut T > , old : * mut T , }
};
}
