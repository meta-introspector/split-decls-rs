// Generated macro for EitherErr (type)
macro_rules! Depcrate_future_try_selectEitherErr {
() => {
// Module: crate::future::try_select
// Provides: {"EitherErr"}
// Dependencies: {}
type EitherErr < A , B > = Either < (< A as TryFuture > :: Error , B) , (< B as TryFuture > :: Error , A) > ;
};
}
