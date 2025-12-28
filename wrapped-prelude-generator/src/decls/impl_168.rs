macro_rules! deps {
    () => {
        DependencyAnalysisVisitor!();
        VernacularWalk!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < 'ast > VernacularWalk < 'ast > for DependencyAnalysisVisitor { fn walk_signature (& mut self , i : & 'ast Signature) { syn :: visit :: visit_signature (self , i) ; } fn walk_block (& mut self , i : & 'ast Block) { syn :: visit :: visit_block (self , i) ; } fn walk_attribute (& mut self , i : & 'ast Attribute) { syn :: visit :: visit_attribute (self , i) ; } fn walk_fields (& mut self , i : & 'ast Fields) { syn :: visit :: visit_fields (self , i) ; } fn walk_item_enum (& mut self , i : & 'ast ItemEnum) { syn :: visit :: visit_item_enum (self , i) ; } fn walk_item_trait (& mut self , i : & 'ast ItemTrait) { syn :: visit :: visit_item_trait (self , i) ; } fn walk_item_type (& mut self , i : & 'ast ItemType) { syn :: visit :: visit_item_type (self , i) ; } fn walk_item_union (& mut self , i : & 'ast ItemUnion) { syn :: visit :: visit_item_union (self , i) ; } fn walk_item_const (& mut self , i : & 'ast ItemConst) { syn :: visit :: visit_item_const (self , i) ; } fn walk_item_static (& mut self , i : & 'ast ItemStatic) { syn :: visit :: visit_item_static (self , i) ; } fn walk_item_macro (& mut self , i : & 'ast ItemMacro) { syn :: visit :: visit_item_macro (self , i) ; } fn walk_item_mod (& mut self , i : & 'ast ItemMod) { syn :: visit :: visit_item_mod (self , i) ; } fn walk_return_type (& mut self , i : & 'ast ReturnType) { syn :: visit :: visit_return_type (self , i) ; } fn walk_fn_arg (& mut self , i : & 'ast FnArg) { syn :: visit :: visit_fn_arg (self , i) ; } fn walk_path (& mut self , i : & 'ast syn :: Path) { syn :: visit :: visit_path (self , i) ; } fn walk_type (& mut self , i : & 'ast syn :: Type) { syn :: visit :: visit_type (self , i) ; } fn walk_bare_fn (& mut self , i : & 'ast syn :: TypeBareFn) { syn :: visit :: visit_type_bare_fn (self , i) ; } fn walk_macro (& mut self , i : & 'ast syn :: Macro) { syn :: visit :: visit_macro (self , i) ; } fn walk_type_path (& mut self , i : & 'ast syn :: TypePath) { syn :: visit :: visit_type_path (self , i) ; } fn walk_type_param_bound (& mut self , i : & 'ast syn :: TypeParamBound) { syn :: visit :: visit_type_param_bound (self , i) ; } fn walk_variant (& mut self , i : & 'ast syn :: Variant) { syn :: visit :: visit_variant (self , i) ; } fn walk_trait_item (& mut self , i : & 'ast syn :: TraitItem) { syn :: visit :: visit_trait_item (self , i) ; } fn walk_expr (& mut self , i : & 'ast syn :: Expr) { syn :: visit :: visit_expr (self , i) ; } }
    };
}

impl_168!();