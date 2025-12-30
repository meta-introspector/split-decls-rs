// Generated macro for impl_93 (impl)
macro_rules! Depcrate_interact_actions_lookupimpl_93 {
() => {
// Module: crate::interact::actions::lookup
// Provides: {"impl_93"}
// Dependencies: {}
impl Lookup { # [doc = " Create a lookup object."] pub fn new () -> Self { Self { buf : Vec :: new () } } # [doc = " Checks whethere the buffer will be matched and returns [`Captures`] in such case."] pub fn on < N > (& mut self , buf : & [u8] , eof : bool , pattern : N) -> Result < Option < Captures > , Error > where N : Needle , { self . buf . extend (buf) ; check (& mut self . buf , pattern , eof) } # [doc = " Cleans internal buffer."] # [doc = ""] # [doc = " So the next [`Lookup::on`] can be called with no other data involved."] pub fn clear (& mut self) { self . buf . clear () ; } }
};
}
