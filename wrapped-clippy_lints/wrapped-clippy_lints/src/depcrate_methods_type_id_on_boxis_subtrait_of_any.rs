// Generated macro for is_subtrait_of_any (function)
macro_rules! Depcrate_methods_type_id_on_boxis_subtrait_of_any {
() => {
// Module: crate::methods::type_id_on_box
// Provides: {"is_subtrait_of_any"}
// Dependencies: {}
# [doc = " Checks if the given type is `dyn Any`, or a trait object that has `Any` as a supertrait."] # [doc = " Only in those cases will its vtable have a `type_id` method that returns the implementor's"] # [doc = " `TypeId`, and only in those cases can we give a proper suggestion to dereference the box."] # [doc = ""] # [doc = " If this returns false, then `.type_id()` likely (this may have FNs) will not be what the user"] # [doc = " expects in any case and dereferencing it won't help either. It will likely require some"] # [doc = " other changes, but it is still worth emitting a lint."] # [doc = " See <https://github.com/rust-lang/rust-clippy/pull/11350#discussion_r1544863005> for more details."] fn is_subtrait_of_any (cx : & LateContext < '_ > , ty : Ty < '_ >) -> bool { if let ty :: Dynamic (preds , ..) = ty . kind () { preds . iter () . any (| p | match p . skip_binder () { ExistentialPredicate :: Trait (tr) => { cx . tcx . is_diagnostic_item (sym :: Any , tr . def_id) || cx . tcx . explicit_super_predicates_of (tr . def_id) . iter_identity_copied () . any (| (clause , _) | { matches ! (clause . kind () . skip_binder () , ty :: ClauseKind :: Trait (super_tr) if cx . tcx . is_diagnostic_item (sym :: Any , super_tr . def_id ())) }) } , _ => false , }) } else { false } }
};
}
