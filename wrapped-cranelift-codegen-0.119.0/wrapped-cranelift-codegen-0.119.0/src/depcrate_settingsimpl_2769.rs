// Generated macro for impl_2769 (impl)
macro_rules! Depcrate_settingsimpl_2769 {
() => {
// Module: crate::settings
// Provides: {"impl_2769"}
// Dependencies: {}
impl fmt :: Display for SetError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { SetError :: BadName (name) => write ! (f , "No existing setting named '{name}'") , SetError :: BadType => { write ! (f , "Trying to set a setting with the wrong type") } SetError :: BadValue (value) => { write ! (f , "Unexpected value for a setting, expected {value}") } } } }
};
}
