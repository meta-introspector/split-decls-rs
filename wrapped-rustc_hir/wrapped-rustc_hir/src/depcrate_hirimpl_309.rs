// Generated macro for impl_309 (impl)
macro_rules! Depcrate_hirimpl_309 {
() => {
// Module: crate::hir
// Provides: {"impl_309"}
// Dependencies: {}
# [doc = " A colloquial, trivially pluralizable description of this const context for use in error"] # [doc = " messages."] impl fmt :: Display for ConstContext { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Self :: Const { .. } => write ! (f , "constant") , Self :: Static (_) => write ! (f , "static") , Self :: ConstFn => write ! (f , "constant function") , } } }
};
}
