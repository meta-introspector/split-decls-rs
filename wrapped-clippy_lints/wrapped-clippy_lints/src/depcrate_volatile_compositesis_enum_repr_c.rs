// Generated macro for is_enum_repr_c (function)
macro_rules! Depcrate_volatile_compositesis_enum_repr_c {
() => {
// Module: crate::volatile_composites
// Provides: {"is_enum_repr_c"}
// Dependencies: {}
# [doc = " Enum with some fixed representation and no data-carrying variants."] fn is_enum_repr_c < 'tcx > (_cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> bool { ty . ty_adt_def () . is_some_and (| adt_def | { adt_def . is_enum () && adt_def . repr () . inhibit_struct_field_reordering () && adt_def . is_payloadfree () }) }
};
}
