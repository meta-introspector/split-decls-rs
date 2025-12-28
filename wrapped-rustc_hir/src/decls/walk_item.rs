macro_rules! deps {
    () => {
        UseKind!();
        Visitor!();
        ItemKind!();
        Mod!();
        Item!();
        TraitImplHeader!();
        Impl!();
        FnKind!();
    };
}

macro_rules! walk_item {
    () => {
        deps!();
        pub fn walk_item < 'v , V : Visitor < 'v > > (visitor : & mut V , item : & 'v Item < 'v >) -> V :: Result { let Item { owner_id : _ , kind , span : _ , vis_span : _ , has_delayed_lints : _ } = item ; try_visit ! (visitor . visit_id (item . hir_id ())) ; match * kind { ItemKind :: ExternCrate (orig_name , ident) => { visit_opt ! (visitor , visit_name , orig_name) ; try_visit ! (visitor . visit_ident (ident)) ; } ItemKind :: Use (ref path , kind) => { try_visit ! (visitor . visit_use (path , item . hir_id ())) ; match kind { UseKind :: Single (ident) => try_visit ! (visitor . visit_ident (ident)) , UseKind :: Glob | UseKind :: ListStem => { } } } ItemKind :: Static (_ , ident , ref typ , body) => { try_visit ! (visitor . visit_ident (ident)) ; try_visit ! (visitor . visit_ty_unambig (typ)) ; try_visit ! (visitor . visit_nested_body (body)) ; } ItemKind :: Const (ident , ref generics , ref typ , body) => { try_visit ! (visitor . visit_ident (ident)) ; try_visit ! (visitor . visit_generics (generics)) ; try_visit ! (visitor . visit_ty_unambig (typ)) ; try_visit ! (visitor . visit_nested_body (body)) ; } ItemKind :: Fn { ident , sig , generics , body : body_id , .. } => { try_visit ! (visitor . visit_ident (ident)) ; try_visit ! (visitor . visit_fn (FnKind :: ItemFn (ident , generics , sig . header) , sig . decl , body_id , item . span , item . owner_id . def_id ,)) ; } ItemKind :: Macro (ident , _def , _kind) => { try_visit ! (visitor . visit_ident (ident)) ; } ItemKind :: Mod (ident , ref module) => { try_visit ! (visitor . visit_ident (ident)) ; try_visit ! (visitor . visit_mod (module , item . span , item . hir_id ())) ; } ItemKind :: ForeignMod { abi : _ , items } => { walk_list ! (visitor , visit_foreign_item_ref , items) ; } ItemKind :: GlobalAsm { asm : _ , fake_body } => { try_visit ! (visitor . visit_nested_body (fake_body)) ; } ItemKind :: TyAlias (ident , ref generics , ref ty) => { try_visit ! (visitor . visit_ident (ident)) ; try_visit ! (visitor . visit_generics (generics)) ; try_visit ! (visitor . visit_ty_unambig (ty)) ; } ItemKind :: Enum (ident , ref generics , ref enum_definition) => { try_visit ! (visitor . visit_ident (ident)) ; try_visit ! (visitor . visit_generics (generics)) ; try_visit ! (visitor . visit_enum_def (enum_definition)) ; } ItemKind :: Impl (Impl { generics , of_trait , self_ty , items }) => { try_visit ! (visitor . visit_generics (generics)) ; if let Some (TraitImplHeader { constness : _ , safety : _ , polarity : _ , defaultness : _ , defaultness_span : _ , trait_ref , }) = of_trait { try_visit ! (visitor . visit_trait_ref (trait_ref)) ; } try_visit ! (visitor . visit_ty_unambig (self_ty)) ; walk_list ! (visitor , visit_impl_item_ref , items) ; } ItemKind :: Struct (ident , ref generics , ref struct_definition) | ItemKind :: Union (ident , ref generics , ref struct_definition) => { try_visit ! (visitor . visit_ident (ident)) ; try_visit ! (visitor . visit_generics (generics)) ; try_visit ! (visitor . visit_variant_data (struct_definition)) ; } ItemKind :: Trait (_constness , _is_auto , _safety , ident , ref generics , bounds , trait_item_refs ,) => { try_visit ! (visitor . visit_ident (ident)) ; try_visit ! (visitor . visit_generics (generics)) ; walk_list ! (visitor , visit_param_bound , bounds) ; walk_list ! (visitor , visit_trait_item_ref , trait_item_refs) ; } ItemKind :: TraitAlias (ident , ref generics , bounds) => { try_visit ! (visitor . visit_ident (ident)) ; try_visit ! (visitor . visit_generics (generics)) ; walk_list ! (visitor , visit_param_bound , bounds) ; } } V :: Result :: output () }
    };
}

walk_item!();