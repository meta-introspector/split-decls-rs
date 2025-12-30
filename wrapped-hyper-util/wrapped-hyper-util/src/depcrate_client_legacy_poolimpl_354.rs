// Generated macro for impl_354 (impl)
macro_rules! Depcrate_client_legacy_poolimpl_354 {
() => {
// Module: crate::client::legacy::pool
// Provides: {"impl_354"}
// Dependencies: {}
impl < 'a , T : Poolable + 'a , K : Debug > IdlePopper < 'a , T , K > { fn pop (self , expiration : & Expiration , now : Instant) -> Option < Idle < T > > { while let Some (entry) = self . list . pop () { if ! entry . value . is_open () { trace ! ("removing closed connection for {:?}" , self . key) ; continue ; } if expiration . expires (entry . idle_at , now) { trace ! ("removing expired connection for {:?}" , self . key) ; continue ; } let value = match entry . value . reserve () { # [cfg (feature = "http2")] Reservation :: Shared (to_reinsert , to_checkout) => { self . list . push (Idle { idle_at : now , value : to_reinsert , }) ; to_checkout } Reservation :: Unique (unique) => unique , } ; return Some (Idle { idle_at : entry . idle_at , value , }) ; } None } }
};
}
