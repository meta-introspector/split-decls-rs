// Generated macro for forward_impl_to_listener (macro)
macro_rules! Depcrateforward_impl_to_listener {
() => {
// Module: crate
// Provides: {"forward_impl_to_listener"}
// Dependencies: {}
# [doc = " Implement the `Listener` trait using the underlying `InnerListener`."] macro_rules ! forward_impl_to_listener { ($ gen : ident => $ ty : ty) => { impl <$ gen > crate :: Listener <$ gen > for $ ty { # [cfg (all (feature = "std" , not (target_family = "wasm")))] fn wait (mut self) -> $ gen { self . listener_mut () . wait_internal (None) . unwrap () } # [cfg (all (feature = "std" , not (target_family = "wasm")))] fn wait_timeout (mut self , timeout : std :: time :: Duration) -> Option <$ gen > { self . listener_mut () . wait_internal (std :: time :: Instant :: now () . checked_add (timeout)) } # [cfg (all (feature = "std" , not (target_family = "wasm")))] fn wait_deadline (mut self , deadline : std :: time :: Instant) -> Option <$ gen > { self . listener_mut () . wait_internal (Some (deadline)) } fn discard (mut self) -> bool { self . listener_mut () . discard () } # [inline] fn listens_to (& self , event : & Event <$ gen >) -> bool { core :: ptr :: eq ::< Inner <$ gen >> (&* self . listener () . event , event . inner . load (core :: sync :: atomic :: Ordering :: Acquire) ,) } # [inline] fn same_event (& self , other : &$ ty) -> bool { core :: ptr :: eq ::< Inner <$ gen >> (&* self . listener () . event , &* other . listener () . event) } } impl <$ gen > Future for $ ty { type Output = $ gen ; # [inline] fn poll (mut self : Pin <& mut Self >, cx : & mut Context <'_ >) -> Poll <$ gen > { self . listener_mut () . poll_internal (cx) } } } ; }
};
}
