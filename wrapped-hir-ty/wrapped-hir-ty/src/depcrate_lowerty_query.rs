// Generated macro for ty_query (function)
macro_rules! Depcrate_lowerty_query {
() => {
// Module: crate::lower
// Provides: {"ty_query"}
// Dependencies: {}
# [doc = " Build the declared type of an item. This depends on the namespace; e.g. for"] # [doc = " `struct Foo(usize)`, we have two types: The type of the struct itself, and"] # [doc = " the constructor function `(usize) -> Foo` which lives in the values"] # [doc = " namespace."] pub (crate) fn ty_query < 'db > (db : & 'db dyn HirDatabase , def : TyDefId) -> EarlyBinder < 'db , Ty < 'db > > { let interner = DbInterner :: new_with (db , None , None) ; match def { TyDefId :: BuiltinType (it) => EarlyBinder :: bind (Ty :: from_builtin_type (interner , it)) , TyDefId :: AdtId (it) => EarlyBinder :: bind (Ty :: new_adt (interner , it , GenericArgs :: identity_for_item (interner , it . into ()) ,)) , TyDefId :: TypeAliasId (it) => db . type_for_type_alias_with_diagnostics (it) . 0 , } }
};
}
