// Generated macro for impl_1208 (impl)
macro_rules! Depcrate_core_configimpl_1208 {
() => {
// Module: crate::core::config
// Provides: {"impl_1208"}
// Dependencies: {}
# [doc = " Suitable for passing to `-C debuginfo`"] impl Display for DebuginfoLevel { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { use DebuginfoLevel :: * ; f . write_str (match self { None => "0" , LineDirectivesOnly => "line-directives-only" , LineTablesOnly => "line-tables-only" , Limited => "1" , Full => "2" , }) } }
};
}
