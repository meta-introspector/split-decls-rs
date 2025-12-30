// Generated macro for impl_246 (impl)
macro_rules! Depcrate_parse_errorimpl_246 {
() => {
// Module: crate::parse::error
// Provides: {"impl_246"}
// Dependencies: {}
impl Display for ParseNode { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Self :: SectionHeader => write ! (f , "section header") , Self :: Name => write ! (f , "name") , Self :: Value => write ! (f , "value") , } } }
};
}
