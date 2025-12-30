// Generated macro for is_manually_drop (function)
macro_rules! Depcrate_tyis_manually_drop {
() => {
// Module: crate::ty
// Provides: {"is_manually_drop"}
// Dependencies: {}
# [doc = " Checks if the type is `core::mem::ManuallyDrop<_>`"] pub fn is_manually_drop (ty : Ty < '_ >) -> bool { ty . ty_adt_def () . is_some_and (AdtDef :: is_manually_drop) }
};
}
