// Generated macro for lower_variant (function)
macro_rules! Depcrate_collectlower_variant {
() => {
// Module: crate::collect
// Provides: {"lower_variant"}
// Dependencies: {}
fn lower_variant < 'tcx > (tcx : TyCtxt < 'tcx > , variant_did : Option < LocalDefId > , ident : Ident , discr : ty :: VariantDiscr , def : & hir :: VariantData < 'tcx > , adt_kind : ty :: AdtKind , parent_did : LocalDefId ,) -> ty :: VariantDef { let mut field_uniqueness_check_ctx = FieldUniquenessCheckContext :: new (tcx) ; let fields = def . fields () . iter () . inspect (| field | { field_uniqueness_check_ctx . check_field_decl (field . ident , field . span . into ()) ; }) . map (| f | ty :: FieldDef { did : f . def_id . to_def_id () , name : f . ident . name , vis : tcx . visibility (f . def_id) , safety : f . safety , value : f . default . map (| v | v . def_id . to_def_id ()) , }) . collect () ; let recovered = match def { hir :: VariantData :: Struct { recovered : Recovered :: Yes (guar) , .. } => Some (* guar) , _ => None , } ; ty :: VariantDef :: new (ident . name , variant_did . map (LocalDefId :: to_def_id) , def . ctor () . map (| (kind , _ , def_id) | (kind , def_id . to_def_id ())) , discr , fields , parent_did . to_def_id () , recovered , adt_kind == AdtKind :: Struct && find_attr ! (tcx . get_all_attrs (parent_did) , AttributeKind :: NonExhaustive (..)) || variant_did . is_some_and (| variant_did | { find_attr ! (tcx . get_all_attrs (variant_did) , AttributeKind :: NonExhaustive (..)) }) ,) }
};
}
