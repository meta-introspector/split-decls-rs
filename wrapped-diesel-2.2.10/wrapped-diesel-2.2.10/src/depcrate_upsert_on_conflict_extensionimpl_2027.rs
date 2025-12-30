// Generated macro for impl_2027 (impl)
macro_rules! Depcrate_upsert_on_conflict_extensionimpl_2027 {
() => {
// Module: crate::upsert::on_conflict_extension
// Provides: {"impl_2027"}
// Dependencies: {}
impl < Stmt , T , P > DecoratableTarget < P > for IncompleteOnConflict < Stmt , T > where P : Expression , P :: SqlType : BoolOrNullableBool , T : DecoratableTarget < P > , { type FilterOutput = IncompleteOnConflict < Stmt , < T as DecoratableTarget < P > > :: FilterOutput > ; fn filter_target (self , predicate : P) -> Self :: FilterOutput { IncompleteOnConflict { stmt : self . stmt , target : self . target . filter_target (predicate) , } } }
};
}
