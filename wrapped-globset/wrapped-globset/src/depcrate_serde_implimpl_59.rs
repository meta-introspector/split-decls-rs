// Generated macro for impl_59 (impl)
macro_rules! Depcrate_serde_implimpl_59 {
() => {
// Module: crate::serde_impl
// Provides: {"impl_59"}
// Dependencies: {}
impl < 'de > Visitor < 'de > for GlobVisitor { type Value = Glob ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter ,) -> std :: fmt :: Result { formatter . write_str ("a glob pattern") } fn visit_str < E > (self , v : & str) -> Result < Self :: Value , E > where E : Error , { Glob :: new (v) . map_err (serde :: de :: Error :: custom) } }
};
}
