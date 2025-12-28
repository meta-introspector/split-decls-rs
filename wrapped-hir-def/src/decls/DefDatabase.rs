macro_rules! deps {
    () => {
        AssocItemId!();
        TraitSignature!();
        TypeAliasSignature!();
        DefWithBodyId!();
        StaticSignature!();
        InternDatabase!();
        GenericDefId!();
        LocalFieldId!();
        StructSignature!();
        Visibility!();
        AttrsWithOwner!();
        ImportMap!();
        EnumSignature!();
        ItemTree!();
        Attrs!();
        VariantId!();
        ExpressionStore!();
        ImplSignature!();
        ConstSignature!();
        ExpressionStoreSourceMap!();
        UnionSignature!();
        FunctionSignature!();
        AttrDefId!();
        MacroId!();
    };
}

macro_rules! DefDatabase {
    () => {
        deps!();
        # [query_group :: query_group] pub trait DefDatabase : InternDatabase + ExpandDatabase + SourceDatabase { # [doc = " Whether to expand procedural macros during name resolution."] # [salsa :: input] fn expand_proc_attr_macros (& self) -> bool ; # [doc = " Computes an [`ItemTree`] for the given file or macro expansion."] # [salsa :: invoke (file_item_tree_query)] # [salsa :: transparent] fn file_item_tree (& self , file_id : HirFileId) -> & ItemTree ; # [doc = " Turns a MacroId into a MacroDefId, describing the macro's definition post name resolution."] # [salsa :: invoke (macro_def)] fn macro_def (& self , m : MacroId) -> MacroDefId ; # [salsa :: tracked] fn trait_signature (& self , trait_ : TraitId) -> Arc < TraitSignature > { self . trait_signature_with_source_map (trait_) . 0 } # [salsa :: tracked] fn impl_signature (& self , impl_ : ImplId) -> Arc < ImplSignature > { self . impl_signature_with_source_map (impl_) . 0 } # [salsa :: tracked] fn struct_signature (& self , struct_ : StructId) -> Arc < StructSignature > { self . struct_signature_with_source_map (struct_) . 0 } # [salsa :: tracked] fn union_signature (& self , union_ : UnionId) -> Arc < UnionSignature > { self . union_signature_with_source_map (union_) . 0 } # [salsa :: tracked] fn enum_signature (& self , e : EnumId) -> Arc < EnumSignature > { self . enum_signature_with_source_map (e) . 0 } # [salsa :: tracked] fn const_signature (& self , e : ConstId) -> Arc < ConstSignature > { self . const_signature_with_source_map (e) . 0 } # [salsa :: tracked] fn static_signature (& self , e : StaticId) -> Arc < StaticSignature > { self . static_signature_with_source_map (e) . 0 } # [salsa :: tracked] fn function_signature (& self , e : FunctionId) -> Arc < FunctionSignature > { self . function_signature_with_source_map (e) . 0 } # [salsa :: tracked] fn type_alias_signature (& self , e : TypeAliasId) -> Arc < TypeAliasSignature > { self . type_alias_signature_with_source_map (e) . 0 } # [salsa :: invoke (TraitSignature :: query)] fn trait_signature_with_source_map (& self , trait_ : TraitId ,) -> (Arc < TraitSignature > , Arc < ExpressionStoreSourceMap >) ; # [salsa :: invoke (ImplSignature :: query)] fn impl_signature_with_source_map (& self , impl_ : ImplId ,) -> (Arc < ImplSignature > , Arc < ExpressionStoreSourceMap >) ; # [salsa :: invoke (StructSignature :: query)] fn struct_signature_with_source_map (& self , struct_ : StructId ,) -> (Arc < StructSignature > , Arc < ExpressionStoreSourceMap >) ; # [salsa :: invoke (UnionSignature :: query)] fn union_signature_with_source_map (& self , union_ : UnionId ,) -> (Arc < UnionSignature > , Arc < ExpressionStoreSourceMap >) ; # [salsa :: invoke (EnumSignature :: query)] fn enum_signature_with_source_map (& self , e : EnumId ,) -> (Arc < EnumSignature > , Arc < ExpressionStoreSourceMap >) ; # [salsa :: invoke (ConstSignature :: query)] fn const_signature_with_source_map (& self , e : ConstId ,) -> (Arc < ConstSignature > , Arc < ExpressionStoreSourceMap >) ; # [salsa :: invoke (StaticSignature :: query)] fn static_signature_with_source_map (& self , e : StaticId ,) -> (Arc < StaticSignature > , Arc < ExpressionStoreSourceMap >) ; # [salsa :: invoke (FunctionSignature :: query)] fn function_signature_with_source_map (& self , e : FunctionId ,) -> (Arc < FunctionSignature > , Arc < ExpressionStoreSourceMap >) ; # [salsa :: invoke (TypeAliasSignature :: query)] fn type_alias_signature_with_source_map (& self , e : TypeAliasId ,) -> (Arc < TypeAliasSignature > , Arc < ExpressionStoreSourceMap >) ; # [salsa :: invoke (Body :: body_with_source_map_query)] # [salsa :: lru (512)] fn body_with_source_map (& self , def : DefWithBodyId) -> (Arc < Body > , Arc < BodySourceMap >) ; # [salsa :: invoke (Body :: body_query)] fn body (& self , def : DefWithBodyId) -> Arc < Body > ; # [salsa :: invoke (ExprScopes :: expr_scopes_query)] fn expr_scopes (& self , def : DefWithBodyId) -> Arc < ExprScopes > ; # [salsa :: transparent] # [salsa :: invoke (GenericParams :: new)] fn generic_params (& self , def : GenericDefId) -> Arc < GenericParams > ; # [salsa :: transparent] # [salsa :: invoke (GenericParams :: generic_params_and_store)] fn generic_params_and_store (& self , def : GenericDefId ,) -> (Arc < GenericParams > , Arc < ExpressionStore >) ; # [salsa :: transparent] # [salsa :: invoke (GenericParams :: generic_params_and_store_and_source_map)] fn generic_params_and_store_and_source_map (& self , def : GenericDefId ,) -> (Arc < GenericParams > , Arc < ExpressionStore > , Arc < ExpressionStoreSourceMap >) ; # [salsa :: invoke (Attrs :: fields_attrs_query)] fn fields_attrs (& self , def : VariantId) -> Arc < ArenaMap < LocalFieldId , Attrs > > ; # [salsa :: invoke (crate :: attr :: fields_attrs_source_map)] fn fields_attrs_source_map (& self , def : VariantId ,) -> Arc < ArenaMap < LocalFieldId , AstPtr < Either < ast :: TupleField , ast :: RecordField > > > > ; # [salsa :: invoke_interned (AttrsWithOwner :: attrs_query)] fn attrs (& self , def : AttrDefId) -> Attrs ; # [salsa :: transparent] # [salsa :: invoke (lang_item :: lang_attr)] fn lang_attr (& self , def : AttrDefId) -> Option < LangItem > ; # [salsa :: invoke (ImportMap :: import_map_query)] fn import_map (& self , krate : Crate) -> Arc < ImportMap > ; # [salsa :: invoke (visibility :: field_visibilities_query)] fn field_visibilities (& self , var : VariantId) -> Arc < ArenaMap < LocalFieldId , Visibility > > ; # [salsa :: invoke (visibility :: assoc_visibility_query)] fn assoc_visibility (& self , def : AssocItemId) -> Visibility ; # [salsa :: invoke (crate :: lang_item :: crate_notable_traits)] # [salsa :: transparent] fn crate_notable_traits (& self , krate : Crate) -> Option < & [TraitId] > ; # [salsa :: invoke (crate_supports_no_std)] fn crate_supports_no_std (& self , crate_id : Crate) -> bool ; # [salsa :: invoke (include_macro_invoc)] fn include_macro_invoc (& self , crate_id : Crate) -> Arc < [(MacroCallId , EditionedFileId)] > ; }
    };
}

DefDatabase!();