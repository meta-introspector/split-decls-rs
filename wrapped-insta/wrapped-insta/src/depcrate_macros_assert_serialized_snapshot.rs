// Generated macro for _assert_serialized_snapshot (macro)
macro_rules! Depcrate_macros_assert_serialized_snapshot {
() => {
// Module: crate::macros
// Provides: {"_assert_serialized_snapshot"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! _assert_serialized_snapshot { (format =$ format : ident , $ value : expr , $ (match ..) ? { $ ($ k : expr => $ v : expr) ,* $ (,) ? } $ ($ arg : tt) *) => { { let transform = | value | { $ crate :: _prepare_snapshot_for_redaction ! (value , { $ ($ k => $ v) ,* } , $ format) } ; $ crate :: _assert_snapshot_base ! (transform = transform , $ value $ ($ arg) *) ; } } ; (format =$ format : ident , $ name : expr , $ value : expr , $ (match ..) ? { $ ($ k : expr => $ v : expr) ,* $ (,) ? } , $ debug_expr : expr $ (,) ?) => { { let transform = | value | { $ crate :: _prepare_snapshot_for_redaction ! (value , { $ ($ k => $ v) ,* } , $ format) } ; $ crate :: _assert_snapshot_base ! (transform = transform , $ name , $ value , $ debug_expr) ; } } ; (format =$ format : ident , $ name : expr , $ value : expr , $ (match ..) ? { $ ($ k : expr => $ v : expr) ,* $ (,) ? } $ (,) ?) => { { let transform = | value | { $ crate :: _prepare_snapshot_for_redaction ! (value , { $ ($ k => $ v) ,* } , $ format) } ; $ crate :: _assert_snapshot_base ! (transform = transform , $ name , $ value) ; } } ; (format =$ format : ident , $ ($ arg : tt) *) => { { let transform = | value | { $ crate :: _macro_support :: serialize_value (& value , $ crate :: _macro_support :: SerializationFormat ::$ format ,) } ; $ crate :: _assert_snapshot_base ! (transform = transform , $ ($ arg) *) ; } } ; }
};
}
