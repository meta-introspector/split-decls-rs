// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
# [cfg (all (feature = "std" , not (target_family = "wasm")))] impl Strategy < '_ > for Blocking { type Context = () ; type Future = Ready ; # [inline] fn wait (& mut self , evl : EventListener) -> Self :: Future { evl . wait () ; Ready { _private : () } } # [inline] fn poll < T , L : Listener < T > + Unpin > (& mut self , event_listener : & mut Option < L > , _context : & mut Self :: Context ,) -> Poll < T > { let result = event_listener . take () . expect ("`event_listener` should never be `None`") . wait () ; Poll :: Ready (result) } }
};
}
