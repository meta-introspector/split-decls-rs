// Generated macro for impl_18 (impl)
macro_rules! Depcrate_barrierimpl_18 {
() => {
// Module: crate::barrier
// Provides: {"impl_18"}
// Dependencies: {}
impl EventListenerFuture for BarrierWaitInner < '_ > { type Output = BarrierWaitResult ; fn poll_with_strategy < 'a , S : Strategy < 'a > > (self : Pin < & mut Self > , strategy : & mut S , cx : & mut S :: Context ,) -> Poll < Self :: Output > { let mut this = self . project () ; loop { match this . state { WaitState :: Initial => { let mut state = ready ! (this . lock . as_mut () . as_pin_mut () . unwrap () . poll_with_strategy (strategy , cx)) ; this . lock . as_mut () . set (None) ; let local_gen = state . generation_id ; state . count += 1 ; if state . count < this . barrier . n { * this . evl = Some (this . barrier . event . listen ()) ; * this . state = WaitState :: Waiting { local_gen } ; } else { state . count = 0 ; state . generation_id = state . generation_id . wrapping_add (1) ; this . barrier . event . notify (usize :: MAX) ; return Poll :: Ready (BarrierWaitResult { is_leader : true }) ; } } WaitState :: Waiting { local_gen } => { ready ! (strategy . poll (this . evl , cx)) ; this . lock . as_mut () . set (Some (this . barrier . state . lock ())) ; * this . state = WaitState :: Reacquiring { local_gen : * local_gen , } ; } WaitState :: Reacquiring { local_gen } => { let state = ready ! (this . lock . as_mut () . as_pin_mut () . unwrap () . poll_with_strategy (strategy , cx)) ; this . lock . set (None) ; if * local_gen == state . generation_id && state . count < this . barrier . n { * this . evl = Some (this . barrier . event . listen ()) ; * this . state = WaitState :: Waiting { local_gen : * local_gen , } ; } else { return Poll :: Ready (BarrierWaitResult { is_leader : false }) ; } } } } } }
};
}
