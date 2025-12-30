// Generated macro for impl_70 (impl)
macro_rules! Depcrate_messagesimpl_70 {
() => {
// Module: crate::messages
// Provides: {"impl_70"}
// Dependencies: {}
impl fmt :: Display for ArtifactDebuginfo { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { ArtifactDebuginfo :: None => f . write_char ('0') , ArtifactDebuginfo :: Limited => f . write_char ('1') , ArtifactDebuginfo :: Full => f . write_char ('2') , ArtifactDebuginfo :: LineDirectivesOnly => f . write_str ("line-directives-only") , ArtifactDebuginfo :: LineTablesOnly => f . write_str ("line-tables-only") , ArtifactDebuginfo :: UnknownInt (n) => write ! (f , "{n}") , ArtifactDebuginfo :: UnknownString (s) => f . write_str (s) , } } }
};
}
