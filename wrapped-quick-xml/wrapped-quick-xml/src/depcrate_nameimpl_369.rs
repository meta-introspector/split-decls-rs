// Generated macro for impl_369 (impl)
macro_rules! Depcrate_nameimpl_369 {
() => {
// Module: crate::name
// Provides: {"impl_369"}
// Dependencies: {}
impl Default for NamespaceResolver { fn default () -> Self { let mut buffer = Vec :: new () ; let mut bindings = Vec :: new () ; for ent in & [RESERVED_NAMESPACE_XML , RESERVED_NAMESPACE_XMLNS] { let prefix = ent . 0 . into_inner () ; let uri = ent . 1 . into_inner () ; bindings . push (NamespaceBinding { start : buffer . len () , prefix_len : prefix . len () , value_len : uri . len () , level : 0 , }) ; buffer . extend (prefix) ; buffer . extend (uri) ; } Self { buffer , bindings , nesting_level : 0 , } } }
};
}
