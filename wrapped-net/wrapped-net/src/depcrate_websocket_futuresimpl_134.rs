// Generated macro for impl_134 (impl)
macro_rules! Depcrate_websocket_futuresimpl_134 {
() => {
// Module: crate::websocket::futures
// Provides: {"impl_134"}
// Dependencies: {}
# [pinned_drop] impl PinnedDrop for WebSocket { fn drop (self : Pin < & mut Self >) { self . ws . close () . unwrap () ; for (ty , cb) in [("open" , self . closures . 0 . as_ref ()) , ("message" , self . closures . 1 . as_ref ()) , ("error" , self . closures . 2 . as_ref ()) ,] { let _ = self . ws . remove_event_listener_with_callback (ty , cb . unchecked_ref ()) ; } if let Ok (close_event) = web_sys :: CloseEvent :: new_with_event_init_dict ("close" , web_sys :: CloseEventInit :: new () . code (1000) . reason ("client dropped") ,) { let _ = self . ws . dispatch_event (& close_event) ; } } }
};
}
