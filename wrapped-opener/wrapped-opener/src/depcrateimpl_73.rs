// Generated macro for impl_73 (impl)
macro_rules! Depcrateimpl_73 {
() => {
// Module: crate
// Provides: {"impl_73"}
// Dependencies: {}
impl Error for OpenError { fn source (& self) -> Option < & (dyn Error + 'static) > { match self { OpenError :: Io (inner) => Some (inner) , OpenError :: Spawn { cmds : _ , source } => Some (source) , OpenError :: ExitStatus { .. } => None , } } }
};
}
