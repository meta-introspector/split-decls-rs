// Generated macro for impl_185 (impl)
macro_rules! Depcrate_manifestimpl_185 {
() => {
// Module: crate::manifest
// Provides: {"impl_185"}
// Dependencies: {}
impl Display for TomlDebugInfo { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { TomlDebugInfo :: None => f . write_char ('0') , TomlDebugInfo :: Limited => f . write_char ('1') , TomlDebugInfo :: Full => f . write_char ('2') , TomlDebugInfo :: LineDirectivesOnly => f . write_str ("line-directives-only") , TomlDebugInfo :: LineTablesOnly => f . write_str ("line-tables-only") , } } }
};
}
