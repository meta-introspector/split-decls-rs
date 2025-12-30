// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl < 'a > Strategy < '_ > for NonBlocking < 'a > { type Context = Context < 'a > ; type Future = EventListener ; # [inline] fn wait (& mut self , evl : EventListener) -> Self :: Future { evl } # [inline] fn poll < T , L : Listener < T > + Unpin > (& mut self , event_listener : & mut Option < L > , context : & mut Self :: Context ,) -> Poll < T > { let poll = Pin :: new (event_listener . as_mut () . expect ("`event_listener` should never be `None`") ,) . poll (context) ; if poll . is_ready () { * event_listener = None ; } poll } }
};
}
