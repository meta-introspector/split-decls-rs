// Generated macro for type_for_adt (function)
macro_rules! Depcrate_lowertype_for_adt {
() => {
// Module: crate::lower
// Provides: {"type_for_adt"}
// Dependencies: {}
fn type_for_adt < 'db > (db : & 'db dyn HirDatabase , adt : AdtId) -> EarlyBinder < 'db , Ty < 'db > > { let interner = DbInterner :: new_with (db , None , None) ; let args = GenericArgs :: identity_for_item (interner , adt . into ()) ; let ty = Ty :: new_adt (interner , adt , args) ; EarlyBinder :: bind (ty) }
};
}
