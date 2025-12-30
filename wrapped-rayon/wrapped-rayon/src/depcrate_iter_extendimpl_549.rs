// Generated macro for impl_549 (impl)
macro_rules! Depcrate_iter_extendimpl_549 {
() => {
// Module: crate::iter::extend
// Provides: {"impl_549"}
// Dependencies: {}
impl < T > Reducer < LinkedList < T > > for ListReducer { fn reduce (self , mut left : LinkedList < T > , mut right : LinkedList < T >) -> LinkedList < T > { left . append (& mut right) ; left } }
};
}
