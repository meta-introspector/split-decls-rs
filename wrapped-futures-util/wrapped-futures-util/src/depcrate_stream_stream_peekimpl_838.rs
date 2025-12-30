// Generated macro for impl_838 (impl)
macro_rules! Depcrate_stream_stream_peekimpl_838 {
() => {
// Module: crate::stream::stream::peek
// Provides: {"impl_838"}
// Dependencies: {}
impl < T , Item > FnOnce1 < & Item > for NextIfEqFn < '_ , T , Item > where T : ? Sized , Item : PartialEq < T > , { type Output = bool ; fn call_once (self , next : & Item) -> Self :: Output { next == self . expected } }
};
}
