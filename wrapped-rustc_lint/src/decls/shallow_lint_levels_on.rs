macro_rules! deps {
    () => {
        LintLevelsBuilder!();
        LintLevelQueryMap!();
    };
}

macro_rules! shallow_lint_levels_on {
    () => {
        deps!();
        # [instrument (level = "trace" , skip (tcx) , ret)] fn shallow_lint_levels_on (tcx : TyCtxt < '_ > , owner : hir :: OwnerId) -> ShallowLintLevelMap { let store = unerased_lint_store (tcx . sess) ; let attrs = tcx . hir_attr_map (owner) ; let mut levels = LintLevelsBuilder { sess : tcx . sess , features : tcx . features () , provider : LintLevelQueryMap { tcx , cur : owner . into () , specs : ShallowLintLevelMap :: default () , empty : FxIndexMap :: default () , attrs , } , lint_added_lints : false , store , registered_tools : tcx . registered_tools (()) , } ; if owner == hir :: CRATE_OWNER_ID { levels . add_command_line () ; } match attrs . map . range (..) { [] => { } & [(local_id , _)] => levels . add_id (HirId { owner , local_id }) , _ => match tcx . hir_owner_node (owner) { hir :: OwnerNode :: Item (item) => levels . visit_item (item) , hir :: OwnerNode :: ForeignItem (item) => levels . visit_foreign_item (item) , hir :: OwnerNode :: TraitItem (item) => levels . visit_trait_item (item) , hir :: OwnerNode :: ImplItem (item) => levels . visit_impl_item (item) , hir :: OwnerNode :: Crate (mod_) => { levels . add_id (hir :: CRATE_HIR_ID) ; levels . visit_mod (mod_ , mod_ . spans . inner_span , hir :: CRATE_HIR_ID) } hir :: OwnerNode :: Synthetic => unreachable ! () , } , } let specs = levels . provider . specs ; # [cfg (debug_assertions)] for (_ , v) in specs . specs . iter () { debug_assert ! (! v . is_empty ()) ; } specs }
    };
}

shallow_lint_levels_on!()