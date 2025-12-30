// Generated macro for adt_and_variant_of_res (function)
macro_rules! Depcrate_tyadt_and_variant_of_res {
() => {
// Module: crate::ty
// Provides: {"adt_and_variant_of_res"}
// Dependencies: {}
# [doc = " Gets the struct or enum variant from the given `Res`"] pub fn adt_and_variant_of_res < 'tcx > (cx : & LateContext < 'tcx > , res : Res) -> Option < (AdtDef < 'tcx > , & 'tcx VariantDef) > { match res { Res :: Def (DefKind :: Struct , id) => { let adt = cx . tcx . adt_def (id) ; Some ((adt , adt . non_enum_variant ())) } , Res :: Def (DefKind :: Variant , id) => { let adt = cx . tcx . adt_def (cx . tcx . parent (id)) ; Some ((adt , adt . variant_with_id (id))) } , Res :: Def (DefKind :: Ctor (CtorOf :: Struct , _) , id) => { let adt = cx . tcx . adt_def (cx . tcx . parent (id)) ; Some ((adt , adt . non_enum_variant ())) } , Res :: Def (DefKind :: Ctor (CtorOf :: Variant , _) , id) => { let var_id = cx . tcx . parent (id) ; let adt = cx . tcx . adt_def (cx . tcx . parent (var_id)) ; Some ((adt , adt . variant_with_id (var_id))) } , Res :: SelfCtor (id) => { let adt = cx . tcx . type_of (id) . instantiate_identity () . ty_adt_def () . unwrap () ; Some ((adt , adt . non_enum_variant ())) } , _ => None , } }
};
}
