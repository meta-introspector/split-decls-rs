// Generated macro for impl_333 (impl)
macro_rules! Depcrate_fut_futureimpl_333 {
() => {
// Module: crate::fut::future
// Provides: {"impl_333"}
// Dependencies: {}
impl < F : Future , A : Actor > WrapFuture < A > for F { type Future = FutureWrap < F , A > ; # [doc (hidden)] fn actfuture (self) -> Self :: Future { wrap_future (self) } fn into_actor (self , _ : & A) -> Self :: Future { wrap_future (self) } }
};
}
