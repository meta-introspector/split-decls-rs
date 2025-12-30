// Generated macro for impl_291 (impl)
macro_rules! Depcrate_ast_traitsimpl_291 {
() => {
// Module: crate::ast_traits
// Provides: {"impl_291"}
// Dependencies: {}
impl < Wrapped , Tag > AstNodeWrapper < Wrapped , Tag > { pub fn new (wrapped : Wrapped , _tag : Tag) -> AstNodeWrapper < Wrapped , Tag > { AstNodeWrapper { wrapped , tag : Default :: default () } } pub fn from_mut (wrapped : & mut Wrapped , _tag : Tag) -> & mut AstNodeWrapper < Wrapped , Tag > { unsafe { & mut * < * mut Wrapped > :: cast (wrapped) } } }
};
}
