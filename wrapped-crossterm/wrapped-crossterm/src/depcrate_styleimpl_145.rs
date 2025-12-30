// Generated macro for impl_145 (impl)
macro_rules! Depcrate_styleimpl_145 {
() => {
// Module: crate::style
// Provides: {"impl_145"}
// Dependencies: {}
impl Command for ResetColor { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { f . write_str (csi ! ("0m")) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { sys :: windows :: reset () } }
};
}
