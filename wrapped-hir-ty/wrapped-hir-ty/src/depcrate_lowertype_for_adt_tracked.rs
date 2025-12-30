// Generated macro for type_for_adt_tracked (function)
macro_rules! Depcrate_lowertype_for_adt_tracked {
() => {
// Module: crate::lower
// Provides: {"type_for_adt_tracked"}
// Dependencies: {}
# [salsa_macros :: tracked (cycle_result = type_for_adt_cycle_result)] fn type_for_adt_tracked (db : & dyn HirDatabase , adt : AdtId) -> Binders < Ty > { type_for_adt (db , adt) }
};
}
