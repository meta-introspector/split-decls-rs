macro_rules! deps {
    () => {
        FnKind!();
        Visitor!();
        ImplItemKind!();
        ImplItemImplKind!();
        ImplItem!();
    };
}

macro_rules! walk_impl_item {
    () => {
        deps!();
        pub fn walk_impl_item < 'v , V : Visitor < 'v > > (visitor : & mut V , impl_item : & 'v ImplItem < 'v > ,) -> V :: Result { let ImplItem { owner_id : _ , ident , ref generics , ref impl_kind , ref kind , span : _ , has_delayed_lints : _ , } = * impl_item ; try_visit ! (visitor . visit_ident (ident)) ; try_visit ! (visitor . visit_generics (generics)) ; try_visit ! (visitor . visit_id (impl_item . hir_id ())) ; match impl_kind { ImplItemImplKind :: Inherent { vis_span : _ } => { } ImplItemImplKind :: Trait { defaultness , trait_item_def_id : _ } => { try_visit ! (visitor . visit_defaultness (defaultness)) ; } } match * kind { ImplItemKind :: Const (ref ty , body) => { try_visit ! (visitor . visit_ty_unambig (ty)) ; visitor . visit_nested_body (body) } ImplItemKind :: Fn (ref sig , body_id) => visitor . visit_fn (FnKind :: Method (impl_item . ident , sig) , sig . decl , body_id , impl_item . span , impl_item . owner_id . def_id ,) , ImplItemKind :: Type (ref ty) => visitor . visit_ty_unambig (ty) , } }
    };
}

walk_impl_item!();