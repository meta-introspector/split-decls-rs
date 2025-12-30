// Generated macro for impl_1642 (impl)
macro_rules! Depcrate_stream_select_with_strategyimpl_1642 {
() => {
// Module: crate::stream::select_with_strategy
// Provides: {"impl_1642"}
// Dependencies: {}
impl < St1 , St2 , Clos , State > SelectWithStrategy < St1 , St2 , Clos , State > { # [doc = " Acquires a reference to the underlying streams that this combinator is"] # [doc = " pulling from."] pub fn get_ref (& self) -> (& St1 , & St2) { (& self . stream1 , & self . stream2) } # [doc = " Acquires a mutable reference to the underlying streams that this"] # [doc = " combinator is pulling from."] # [doc = ""] # [doc = " Note that care must be taken to avoid tampering with the state of the"] # [doc = " stream which may otherwise confuse this combinator."] pub fn get_mut (& mut self) -> (& mut St1 , & mut St2) { (& mut self . stream1 , & mut self . stream2) } # [doc = " Acquires a pinned mutable reference to the underlying streams that this"] # [doc = " combinator is pulling from."] # [doc = ""] # [doc = " Note that care must be taken to avoid tampering with the state of the"] # [doc = " stream which may otherwise confuse this combinator."] pub fn get_pin_mut (self : Pin < & mut Self >) -> (Pin < & mut St1 > , Pin < & mut St2 >) { let this = self . project () ; (this . stream1 , this . stream2) } # [doc = " Consumes this combinator, returning the underlying streams."] # [doc = ""] # [doc = " Note that this may discard intermediate state of this combinator, so"] # [doc = " care should be taken to avoid losing resources when this is called."] pub fn into_inner (self) -> (St1 , St2) { (self . stream1 , self . stream2) } }
};
}
