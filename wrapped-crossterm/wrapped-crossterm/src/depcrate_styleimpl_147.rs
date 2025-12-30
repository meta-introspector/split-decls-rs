// Generated macro for impl_147 (impl)
macro_rules! Depcrate_styleimpl_147 {
() => {
// Module: crate::style
// Provides: {"impl_147"}
// Dependencies: {}
impl < T : Display > Command for Print < T > { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { write ! (f , "{}" , self . 0) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { panic ! ("tried to execute Print command using WinAPI, use ANSI instead") ; } # [cfg (windows)] fn is_ansi_code_supported (& self) -> bool { true } }
};
}
