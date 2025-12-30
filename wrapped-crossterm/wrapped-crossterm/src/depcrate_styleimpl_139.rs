// Generated macro for impl_139 (impl)
macro_rules! Depcrate_styleimpl_139 {
() => {
// Module: crate::style
// Provides: {"impl_139"}
// Dependencies: {}
impl Command for SetAttributes { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { for attr in Attribute :: iterator () { if self . 0 . has (attr) { SetAttribute (attr) . write_ansi (f) ? ; } } Ok (()) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { Ok (()) } }
};
}
