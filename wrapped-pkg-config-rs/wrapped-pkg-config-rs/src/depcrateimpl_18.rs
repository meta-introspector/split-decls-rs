// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
# [doc = " Output a command invocation that can be copy-pasted into the terminal."] # [doc = " `Command`'s existing debug implementation is not used for that reason,"] # [doc = " as it can sometimes lead to output such as:"] # [doc = " `PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=\"1\" PKG_CONFIG_ALLOW_SYSTEM_LIBS=\"1\" \"pkg-config\" \"--libs\" \"--cflags\" \"mylibrary\"`"] # [doc = " Which cannot be copy-pasted into terminals such as nushell, and is a bit noisy."] # [doc = " This will look something like:"] # [doc = " `PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1 PKG_CONFIG_ALLOW_SYSTEM_LIBS=1 pkg-config --libs --cflags mylibrary`"] impl Display for WrappedCommand { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let envs = self . env_vars . iter () . map (| (env , arg) | format ! ("{}={}" , env . to_string_lossy () , arg . to_string_lossy ())) . collect :: < Vec < String > > () . join (" ") ; let args = self . args . iter () . map (| arg | quote_if_needed (arg . to_string_lossy () . to_string ())) . collect :: < Vec < String > > () . join (" ") ; write ! (f , "{} {} {}" , envs , self . program . to_string_lossy () , args) } }
};
}
