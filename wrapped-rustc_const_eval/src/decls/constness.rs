macro_rules! constness {
    () => {
        # [doc = " Checks whether a function-like definition is considered to be `const`."] fn constness (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> hir :: Constness { let node = tcx . hir_node_by_def_id (def_id) ; match node { hir :: Node :: Ctor (hir :: VariantData :: Tuple (..)) => hir :: Constness :: Const , hir :: Node :: ForeignItem (item) if let hir :: ForeignItemKind :: Fn (..) = item . kind => { hir :: Constness :: NotConst } hir :: Node :: Expr (e) if let hir :: ExprKind :: Closure (c) = e . kind => c . constness , _ => { if let Some (fn_kind) = node . fn_kind () { if fn_kind . constness () == hir :: Constness :: Const { return hir :: Constness :: Const ; } parent_impl_or_trait_constness (tcx , def_id) } else { tcx . dcx () . span_bug (tcx . def_span (def_id) , format ! ("should not be requesting the constness of items that can't be const: {node:#?}: {:?}" , tcx . def_kind (def_id))) } } } }
    };
}

constness!();