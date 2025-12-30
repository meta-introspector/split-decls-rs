// Generated macro for RenameConflictsVisitor (struct)
macro_rules! Depcrate_semanticsRenameConflictsVisitor {
() => {
// Module: crate::semantics
// Provides: {"RenameConflictsVisitor"}
// Dependencies: {}
struct RenameConflictsVisitor < 'a > { db : & 'a dyn HirDatabase , owner : DefWithBodyId , resolver : Resolver < 'a > , body : & 'a Body , to_be_renamed : BindingId , new_name : Symbol , old_name : Symbol , conflicts : FxHashSet < BindingId > , }
};
}
