// Generated macro for impl_476 (impl)
macro_rules! Depcrate_future_try_selectimpl_476 {
() => {
// Module: crate::future::try_select
// Provides: {"impl_476"}
// Dependencies: {}
impl < A : Unpin , B : Unpin > Future for TrySelect < A , B > where A : TryFuture , B : TryFuture , { type Output = Result < EitherOk < A , B > , EitherErr < A , B > > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let (mut a , mut b) = self . inner . take () . expect ("cannot poll Select twice") ; match a . try_poll_unpin (cx) { Poll :: Ready (Err (x)) => Poll :: Ready (Err (Either :: Left ((x , b)))) , Poll :: Ready (Ok (x)) => Poll :: Ready (Ok (Either :: Left ((x , b)))) , Poll :: Pending => match b . try_poll_unpin (cx) { Poll :: Ready (Err (x)) => Poll :: Ready (Err (Either :: Right ((x , a)))) , Poll :: Ready (Ok (x)) => Poll :: Ready (Ok (Either :: Right ((x , a)))) , Poll :: Pending => { self . inner = Some ((a , b)) ; Poll :: Pending } } , } } }
};
}
