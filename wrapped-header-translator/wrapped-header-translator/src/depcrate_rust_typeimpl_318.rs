// Generated macro for impl_318 (impl)
macro_rules! Depcrate_rust_typeimpl_318 {
() => {
// Module: crate::rust_type
// Provides: {"impl_318"}
// Dependencies: {}
impl Lifetime { fn update (& mut self , new : Self) { match (* self , new) { (_ , Self :: Unspecified) => { } (Self :: Unspecified , _) => { * self = new ; } (Self :: Strong , Self :: Strong) => { } (old , new) => error ! (? old , ? new , "invalid lifetime update") , } } }
};
}
