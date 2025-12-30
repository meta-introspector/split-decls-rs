// Generated macro for type_for_fn (function)
macro_rules! Depcrate_lowertype_for_fn {
() => {
// Module: crate::lower
// Provides: {"type_for_fn"}
// Dependencies: {}
# [doc = " Build the declared type of a function. This should not need to look at the"] # [doc = " function body."] fn type_for_fn < 'db > (db : & 'db dyn HirDatabase , def : FunctionId) -> EarlyBinder < 'db , Ty < 'db > > { let interner = DbInterner :: new_with (db , None , None) ; EarlyBinder :: bind (Ty :: new_fn_def (interner , CallableDefId :: FunctionId (def) . into () , GenericArgs :: identity_for_item (interner , def . into ()) ,)) }
};
}
