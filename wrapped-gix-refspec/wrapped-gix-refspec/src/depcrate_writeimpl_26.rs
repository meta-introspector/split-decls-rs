// Generated macro for impl_26 (impl)
macro_rules! Depcrate_writeimpl_26 {
() => {
// Module: crate::write
// Provides: {"impl_26"}
// Dependencies: {}
impl RefSpecRef < '_ > { # [doc = " Reproduce ourselves in parseable form."] pub fn to_bstring (& self) -> BString { let mut buf = Vec :: with_capacity (128) ; self . write_to (& mut buf) . expect ("no io error") ; buf . into () } # [doc = " Serialize ourselves in a parseable format to `out`."] pub fn write_to (& self , out : & mut dyn std :: io :: Write) -> std :: io :: Result < () > { self . instruction () . write_to (out) } }
};
}
