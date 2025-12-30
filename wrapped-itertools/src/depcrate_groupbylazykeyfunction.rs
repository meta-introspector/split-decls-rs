// Generated macro for KeyFunction (trait)
macro_rules! Depcrate_groupbylazyKeyFunction {
() => {
// Module: crate::groupbylazy
// Provides: {"KeyFunction"}
// Dependencies: {}
# [doc = " A trait to unify `FnMut` for `ChunkBy` with the chunk key in `IntoChunks`"] trait KeyFunction < A > { type Key ; fn call_mut (& mut self , arg : A) -> Self :: Key ; }
};
}
