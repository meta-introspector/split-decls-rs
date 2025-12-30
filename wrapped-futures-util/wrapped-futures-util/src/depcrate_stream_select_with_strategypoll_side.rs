// Generated macro for poll_side (function)
macro_rules! Depcrate_stream_select_with_strategypoll_side {
() => {
// Module: crate::stream::select_with_strategy
// Provides: {"poll_side"}
// Dependencies: {}
# [inline] fn poll_side < St1 , St2 , Clos , State > (select : & mut SelectWithStrategyProj < '_ , St1 , St2 , Clos , State > , side : PollNext , cx : & mut Context < '_ > ,) -> Poll < Option < St1 :: Item > > where St1 : Stream , St2 : Stream < Item = St1 :: Item > , { match side { PollNext :: Left => select . stream1 . as_mut () . poll_next (cx) , PollNext :: Right => select . stream2 . as_mut () . poll_next (cx) , } }
};
}
