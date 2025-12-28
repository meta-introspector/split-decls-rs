macro_rules! deps {
    () => {
        ItemCtxt!();
    };
}

macro_rules! fn_sig {
    () => {
        deps!();
        # [instrument (level = "debug" , skip (tcx) , ret)] fn fn_sig (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> ty :: EarlyBinder < '_ , ty :: PolyFnSig < '_ > > { use rustc_hir :: Node :: * ; use rustc_hir :: * ; let hir_id = tcx . local_def_id_to_hir_id (def_id) ; let icx = ItemCtxt :: new (tcx , def_id) ; let output = match tcx . hir_node (hir_id) { TraitItem (hir :: TraitItem { kind : TraitItemKind :: Fn (sig , TraitFn :: Provided (_)) , generics , .. }) | Item (hir :: Item { kind : ItemKind :: Fn { sig , generics , .. } , .. }) => { lower_fn_sig_recovering_infer_ret_ty (& icx , sig , generics , def_id) } ImplItem (hir :: ImplItem { kind : ImplItemKind :: Fn (sig , _) , generics , .. }) => { if let Item (hir :: Item { kind : ItemKind :: Impl (i) , .. }) = tcx . parent_hir_node (hir_id) && i . of_trait . is_some () { icx . lowerer () . lower_fn_ty (hir_id , sig . header . safety () , sig . header . abi , sig . decl , Some (generics) , None ,) } else { lower_fn_sig_recovering_infer_ret_ty (& icx , sig , generics , def_id) } } TraitItem (hir :: TraitItem { kind : TraitItemKind :: Fn (FnSig { header , decl , span : _ } , _) , generics , .. }) => icx . lowerer () . lower_fn_ty (hir_id , header . safety () , header . abi , decl , Some (generics) , None ,) , ForeignItem (& hir :: ForeignItem { kind : ForeignItemKind :: Fn (sig , _ , _) , .. }) => { let abi = tcx . hir_get_foreign_abi (hir_id) ; compute_sig_of_foreign_fn_decl (tcx , def_id , sig . decl , abi , sig . header . safety ()) } Ctor (data) => { assert_matches ! (data . ctor () , Some (_)) ; let adt_def_id = tcx . hir_get_parent_item (hir_id) . def_id . to_def_id () ; let ty = tcx . type_of (adt_def_id) . instantiate_identity () ; let inputs = data . fields () . iter () . map (| f | tcx . type_of (f . def_id) . instantiate_identity ()) ; let safety = match tcx . layout_scalar_valid_range (adt_def_id) { (Bound :: Unbounded , Bound :: Unbounded) => hir :: Safety :: Safe , _ => hir :: Safety :: Unsafe , } ; ty :: Binder :: dummy (tcx . mk_fn_sig (inputs , ty , false , safety , ExternAbi :: Rust)) } Expr (& hir :: Expr { kind : hir :: ExprKind :: Closure { .. } , .. }) => { bug ! ("to get the signature of a closure, use `args.as_closure().sig()` not `fn_sig()`" ,) ; } x => { bug ! ("unexpected sort of node in fn_sig(): {:?}" , x) ; } } ; ty :: EarlyBinder :: bind (output) }
    };
}

fn_sig!();