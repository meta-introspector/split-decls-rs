// Generated macro for impl_351 (impl)
macro_rules! Depcrate_groupbylazyimpl_351 {
() => {
// Module: crate::groupbylazy
// Provides: {"impl_351"}
// Dependencies: {}
impl < A , K , F > KeyFunction < A > for F where F : FnMut (A) -> K + ? Sized , { type Key = K ; # [inline] fn call_mut (& mut self , arg : A) -> Self :: Key { (* self) (arg) } }
};
}
