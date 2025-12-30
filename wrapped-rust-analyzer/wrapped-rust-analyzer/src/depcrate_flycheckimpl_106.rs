// Generated macro for impl_106 (impl)
macro_rules! Depcrate_flycheckimpl_106 {
() => {
// Module: crate::flycheck
// Provides: {"impl_106"}
// Dependencies: {}
impl fmt :: Display for FlycheckConfig { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { FlycheckConfig :: CargoCommand { command , .. } => write ! (f , "cargo {command}") , FlycheckConfig :: CustomCommand { command , args , .. } => { let display_args = args . iter () . map (| arg | if arg == SAVED_FILE_PLACEHOLDER { "..." } else { arg }) . collect :: < Vec < _ > > () ; write ! (f , "{command} {}" , display_args . join (" ")) } } } }
};
}
