// Generated macro for impl_133 (impl)
macro_rules! Depcrate_styleimpl_133 {
() => {
// Module: crate::style
// Provides: {"impl_133"}
// Dependencies: {}
impl Command for SetUnderlineColor { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { write ! (f , csi ! ("{}m") , Colored :: UnderlineColor (self . 0)) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { Err (std :: io :: Error :: new (std :: io :: ErrorKind :: Other , "SetUnderlineColor not supported by winapi." ,)) } }
};
}
