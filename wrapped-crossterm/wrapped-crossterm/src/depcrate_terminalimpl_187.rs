// Generated macro for impl_187 (impl)
macro_rules! Depcrate_terminalimpl_187 {
() => {
// Module: crate::terminal
// Provides: {"impl_187"}
// Dependencies: {}
impl Command for Clear { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { f . write_str (match self . 0 { ClearType :: All => csi ! ("2J") , ClearType :: Purge => csi ! ("3J") , ClearType :: FromCursorDown => csi ! ("J") , ClearType :: FromCursorUp => csi ! ("1J") , ClearType :: CurrentLine => csi ! ("2K") , ClearType :: UntilNewLine => csi ! ("K") , }) } # [cfg (windows)] fn execute_winapi (& self) -> io :: Result < () > { sys :: clear (self . 0) } }
};
}
