// Generated macro for impl_9 (impl)
macro_rules! Depcrate_extractimpl_9 {
() => {
// Module: crate::extract
// Provides: {"impl_9"}
// Dependencies: {}
impl < S , R > FromRequest < S > for GraphQLRequest < R > where S : Send + Sync , R : IntoResponse + From < ParseRequestError > , { type Rejection = R ; async fn from_request (req : Request , state : & S) -> Result < Self , Self :: Rejection > { Ok (GraphQLRequest (GraphQLBatchRequest :: < R > :: from_request (req , state) . await ? . 0 . into_single () ? , PhantomData ,)) } }
};
}
