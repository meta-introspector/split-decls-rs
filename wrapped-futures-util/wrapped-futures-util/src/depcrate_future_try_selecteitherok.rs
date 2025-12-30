// Generated macro for EitherOk (type)
macro_rules! Depcrate_future_try_selectEitherOk {
() => {
// Module: crate::future::try_select
// Provides: {"EitherOk"}
// Dependencies: {}
type EitherOk < A , B > = Either < (< A as TryFuture > :: Ok , B) , (< B as TryFuture > :: Ok , A) > ;
};
}
