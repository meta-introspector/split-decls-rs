// Generated macro for delegate_access_inner (macro)
macro_rules! Depcratedelegate_access_inner {
() => {
// Module: crate
// Provides: {"delegate_access_inner"}
// Dependencies: {}
macro_rules ! delegate_access_inner { ($ field : ident , $ inner : ty , ($ ($ ind : tt) *)) => { # [doc = " Acquires a reference to the underlying sink or stream that this combinator is"] # [doc = " pulling from."] pub fn get_ref (& self) -> &$ inner { (& self .$ field) $ ($ ind get_ref ()) * } # [doc = " Acquires a mutable reference to the underlying sink or stream that this"] # [doc = " combinator is pulling from."] # [doc = ""] # [doc = " Note that care must be taken to avoid tampering with the state of the"] # [doc = " sink or stream which may otherwise confuse this combinator."] pub fn get_mut (& mut self) -> & mut $ inner { (& mut self .$ field) $ ($ ind get_mut ()) * } # [doc = " Acquires a pinned mutable reference to the underlying sink or stream that this"] # [doc = " combinator is pulling from."] # [doc = ""] # [doc = " Note that care must be taken to avoid tampering with the state of the"] # [doc = " sink or stream which may otherwise confuse this combinator."] pub fn get_pin_mut (self : core :: pin :: Pin <& mut Self >) -> core :: pin :: Pin <& mut $ inner > { self . project () .$ field $ ($ ind get_pin_mut ()) * } # [doc = " Consumes this combinator, returning the underlying sink or stream."] # [doc = ""] # [doc = " Note that this may discard intermediate state of this combinator, so"] # [doc = " care should be taken to avoid losing resources when this is called."] pub fn into_inner (self) -> $ inner { self .$ field $ ($ ind into_inner ()) * } } }
};
}
