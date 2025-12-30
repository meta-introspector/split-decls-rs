// Generated macro for impl_1323 (impl)
macro_rules! Depcrate_stream_try_stream_try_flatten_unorderedimpl_1323 {
() => {
// Module: crate::stream::try_stream::try_flatten_unordered
// Provides: {"impl_1323"}
// Dependencies: {}
impl < St > FlowController < BaseStreamItem < St > , InnerStreamItem < St > > for PropagateBaseStreamError < St > where St : TryStream , St :: Ok : TryStream + Unpin , < St :: Ok as TryStream > :: Error : From < St :: Error > , { fn next_step (item : BaseStreamItem < St >) -> FlowStep < BaseStreamItem < St > , InnerStreamItem < St > > { match item { st @ Either :: Left (_) => FlowStep :: Continue (st) , Either :: Right (mut err) => FlowStep :: Return (err . next_immediate () . unwrap ()) , } } }
};
}
