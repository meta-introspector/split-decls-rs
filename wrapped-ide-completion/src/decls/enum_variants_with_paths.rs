macro_rules! deps {
    () => {
        Completions!();
        CompletionContext!();
        PathKind!();
    };
}

macro_rules! enum_variants_with_paths {
    () => {
        deps!();
        # [doc = " Calls the callback for each variant of the provided enum with the path to the variant."] # [doc = " Skips variants that are visible with single segment paths."] fn enum_variants_with_paths (acc : & mut Completions , ctx : & CompletionContext < '_ > , enum_ : hir :: Enum , impl_ : Option < & ast :: Impl > , cb : impl Fn (& mut Completions , & CompletionContext < '_ > , hir :: Variant , hir :: ModPath) ,) { let mut process_variant = | variant : Variant | { let self_path = hir :: ModPath :: from_segments (hir :: PathKind :: Plain , iter :: once (Name :: new_symbol_root (sym :: Self_)) . chain (iter :: once (variant . name (ctx . db))) ,) ; cb (acc , ctx , variant , self_path) ; } ; let variants = enum_ . variants (ctx . db) ; if let Some (impl_) = impl_ . and_then (| impl_ | ctx . sema . to_def (impl_)) && impl_ . self_ty (ctx . db) . as_adt () == Some (hir :: Adt :: Enum (enum_)) { variants . iter () . for_each (| variant | process_variant (* variant)) ; } for variant in variants { if let Some (path) = ctx . module . find_path (ctx . db , hir :: ModuleDef :: from (variant) , ctx . config . find_path_config (ctx . is_nightly) ,) { if path . segments () . len () > 1 { cb (acc , ctx , variant , path) ; } } } }
    };
}

enum_variants_with_paths!()