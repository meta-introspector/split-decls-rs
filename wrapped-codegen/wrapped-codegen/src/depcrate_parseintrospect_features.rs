// Generated macro for introspect_features (function)
macro_rules! Depcrate_parseintrospect_features {
() => {
// Module: crate::parse
// Provides: {"introspect_features"}
// Dependencies: {}
fn introspect_features (attrs : & [Attribute]) -> types :: Features { let mut ret = types :: Features :: default () ; for attr in attrs { if ! attr . path () . is_ident ("cfg") { continue ; } let features = attr . parse_args_with (parsing :: parse_features) . unwrap () ; if ret . any . is_empty () { ret = features ; } else if ret . any . len () < features . any . len () { assert ! (ret . any . iter () . all (| f | features . any . contains (f))) ; } else { assert ! (features . any . iter () . all (| f | ret . any . contains (f))) ; ret = features ; } } ret }
};
}
