// Generated macro for BUILTIN_ATTRIBUTE_MAP (static)
macro_rules! Depcrate_builtin_attrsBUILTIN_ATTRIBUTE_MAP {
() => {
// Module: crate::builtin_attrs
// Provides: {"BUILTIN_ATTRIBUTE_MAP"}
// Dependencies: {}
pub static BUILTIN_ATTRIBUTE_MAP : LazyLock < FxHashMap < Symbol , & BuiltinAttribute > > = LazyLock :: new (| | { let mut map = FxHashMap :: default () ; for attr in BUILTIN_ATTRIBUTES . iter () { if map . insert (attr . name , attr) . is_some () { panic ! ("duplicate builtin attribute `{}`" , attr . name) ; } } map }) ;
};
}
