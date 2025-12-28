macro_rules! deps {
    () => {
        Completions!();
        TypeLocation!();
        NameRefContext!();
        DotAccess!();
        CompletionContext!();
        ItemListKind!();
        PathKind!();
        NameRefKind!();
    };
}

macro_rules! complete_name_ref {
    () => {
        deps!();
        pub (super) fn complete_name_ref (acc : & mut Completions , ctx : & CompletionContext < '_ > , NameRefContext { nameref , kind } : & NameRefContext < '_ > ,) { match kind { NameRefKind :: Path (path_ctx) => { flyimport :: import_on_the_fly_path (acc , ctx , path_ctx) ; match & path_ctx . kind { PathKind :: Expr { expr_ctx } => { expr :: complete_expr_path (acc , ctx , path_ctx , expr_ctx) ; expr :: complete_expr (acc , ctx) ; dot :: complete_undotted_self (acc , ctx , path_ctx , expr_ctx) ; item_list :: complete_item_list_in_expr (acc , ctx , path_ctx , expr_ctx) ; snippet :: complete_expr_snippet (acc , ctx , path_ctx , expr_ctx) ; } PathKind :: Type { location } => { r#type :: complete_type_path (acc , ctx , path_ctx , location) ; match location { TypeLocation :: TupleField => { field :: complete_field_list_tuple_variant (acc , ctx , path_ctx) ; } TypeLocation :: TypeAscription (ascription) => { r#type :: complete_ascribed_type (acc , ctx , path_ctx , ascription) ; } TypeLocation :: GenericArg { .. } | TypeLocation :: AssocConstEq | TypeLocation :: AssocTypeEq | TypeLocation :: TypeBound | TypeLocation :: ImplTarget | TypeLocation :: ImplTrait | TypeLocation :: Other => () , } } PathKind :: Attr { attr_ctx } => { attribute :: complete_attribute_path (acc , ctx , path_ctx , attr_ctx) ; } PathKind :: Derive { existing_derives } => { attribute :: complete_derive_path (acc , ctx , path_ctx , existing_derives) ; } PathKind :: Item { kind } => { item_list :: complete_item_list (acc , ctx , path_ctx , kind) ; snippet :: complete_item_snippet (acc , ctx , path_ctx , kind) ; if let ItemListKind :: TraitImpl (impl_) = kind { item_list :: trait_impl :: complete_trait_impl_item_by_name (acc , ctx , path_ctx , nameref , impl_ ,) ; } } PathKind :: Pat { .. } => { pattern :: complete_pattern_path (acc , ctx , path_ctx) ; } PathKind :: Vis { has_in_token } => { vis :: complete_vis_path (acc , ctx , path_ctx , has_in_token) ; } PathKind :: Use => { use_ :: complete_use_path (acc , ctx , path_ctx , nameref) ; } } } NameRefKind :: ExternCrate => extern_crate :: complete_extern_crate (acc , ctx) , NameRefKind :: DotAccess (dot_access) => { flyimport :: import_on_the_fly_dot (acc , ctx , dot_access) ; dot :: complete_dot (acc , ctx , dot_access) ; postfix :: complete_postfix (acc , ctx , dot_access) ; } NameRefKind :: Keyword (item) => { keyword :: complete_for_and_where (acc , ctx , item) ; } NameRefKind :: RecordExpr { dot_prefix , expr } => { record :: complete_record_expr_fields (acc , ctx , expr , dot_prefix) ; } NameRefKind :: Pattern (pattern_ctx) => complete_patterns (acc , ctx , pattern_ctx) , } }
    };
}

complete_name_ref!()