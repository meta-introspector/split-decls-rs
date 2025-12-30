// Generated macro for impl_59 (impl)
macro_rules! Depcrate_rawimpl_59 {
() => {
// Module: crate::raw
// Provides: {"impl_59"}
// Dependencies: {}
impl Fallibility { # [doc = " Error to return on capacity overflow."] # [cfg_attr (feature = "inline-more" , inline)] fn capacity_overflow (self) -> TryReserveError { match self { Fallibility :: Fallible => TryReserveError :: CapacityOverflow , Fallibility :: Infallible => panic ! ("Hash table capacity overflow") , } } # [doc = " Error to return on allocation error."] # [cfg_attr (feature = "inline-more" , inline)] fn alloc_err (self , layout : Layout) -> TryReserveError { match self { Fallibility :: Fallible => TryReserveError :: AllocError { layout } , Fallibility :: Infallible => handle_alloc_error (layout) , } } }
};
}
