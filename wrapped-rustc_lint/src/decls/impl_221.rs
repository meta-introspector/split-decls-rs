macro_rules! deps {
    () => {
        BuiltinClashingExternSub!();
        ClashingExternDeclarations!();
        BuiltinClashingExtern!();
    };
}

macro_rules! impl_221 {
    () => {
        deps!();
        impl ClashingExternDeclarations { pub (crate) fn new () -> Self { ClashingExternDeclarations { seen_decls : Default :: default () } } # [doc = " Insert a new foreign item into the seen set. If a symbol with the same name already exists"] # [doc = " for the item, return its HirId without updating the set."] fn insert (& mut self , tcx : TyCtxt < '_ > , fi : hir :: ForeignItemId) -> Option < hir :: OwnerId > { let did = fi . owner_id . to_def_id () ; let instance = Instance :: new_raw (did , ty :: List :: identity_for_item (tcx , did)) ; let name = Symbol :: intern (tcx . symbol_name (instance) . name) ; if let Some (& existing_id) = self . seen_decls . get (& name) { Some (existing_id) } else { self . seen_decls . insert (name , fi . owner_id) } } # [instrument (level = "trace" , skip (self , tcx))] fn check_foreign_item < 'tcx > (& mut self , tcx : TyCtxt < 'tcx > , this_fi : hir :: ForeignItemId) { let DefKind :: Fn = tcx . def_kind (this_fi . owner_id) else { return } ; let Some (existing_did) = self . insert (tcx , this_fi) else { return } ; let existing_decl_ty = tcx . type_of (existing_did) . skip_binder () ; let this_decl_ty = tcx . type_of (this_fi . owner_id) . instantiate_identity () ; debug ! ("ClashingExternDeclarations: Comparing existing {:?}: {:?} to this {:?}: {:?}" , existing_did , existing_decl_ty , this_fi . owner_id , this_decl_ty) ; if ! structurally_same_type (tcx , ty :: TypingEnv :: non_body_analysis (tcx , this_fi . owner_id) , existing_decl_ty , this_decl_ty ,) { let orig = name_of_extern_decl (tcx , existing_did) ; let this = tcx . item_name (this_fi . owner_id . to_def_id ()) ; let orig = orig . get_name () ; let previous_decl_label = get_relevant_span (tcx , existing_did) ; let mismatch_label = get_relevant_span (tcx , this_fi . owner_id) ; let sub = BuiltinClashingExternSub { tcx , expected : existing_decl_ty , found : this_decl_ty } ; let decorator = if orig == this { BuiltinClashingExtern :: SameName { this , orig , previous_decl_label , mismatch_label , sub , } } else { BuiltinClashingExtern :: DiffName { this , orig , previous_decl_label , mismatch_label , sub , } } ; tcx . emit_node_span_lint (CLASHING_EXTERN_DECLARATIONS , this_fi . hir_id () , mismatch_label , decorator ,) ; } } }
    };
}

impl_221!()