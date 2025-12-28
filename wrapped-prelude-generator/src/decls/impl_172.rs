macro_rules! deps {
    () => {
        TypeCollectorVisitor!();
        VernacularWalk!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        impl < 'ast > VernacularWalk < 'ast > for TypeCollectorVisitor { fn walk_signature (& mut self , _i : & 'ast syn :: Signature) { } fn walk_block (& mut self , _i : & 'ast syn :: Block) { } fn walk_attribute (& mut self , _i : & 'ast syn :: Attribute) { } fn walk_fields (& mut self , _i : & 'ast syn :: Fields) { } fn walk_item_enum (& mut self , _i : & 'ast syn :: ItemEnum) { } fn walk_item_trait (& mut self , _i : & 'ast syn :: ItemTrait) { } fn walk_item_type (& mut self , _i : & 'ast syn :: ItemType) { } fn walk_item_union (& mut self , _i : & 'ast syn :: ItemUnion) { } fn walk_item_const (& mut self , _i : & 'ast syn :: ItemConst) { } fn walk_item_static (& mut self , _i : & 'ast syn :: ItemStatic) { } fn walk_item_macro (& mut self , _i : & 'ast syn :: ItemMacro) { } fn walk_item_mod (& mut self , _i : & 'ast syn :: ItemMod) { } fn walk_return_type (& mut self , _i : & 'ast syn :: ReturnType) { } fn walk_fn_arg (& mut self , _i : & 'ast syn :: FnArg) { } fn walk_path (& mut self , i : & 'ast syn :: Path) { syn :: visit :: visit_path (self , i) ; } fn walk_type (& mut self , i : & 'ast syn :: Type) { syn :: visit :: visit_type (self , i) ; } fn walk_bare_fn (& mut self , i : & 'ast syn :: TypeBareFn) { syn :: visit :: visit_type_bare_fn (self , i) ; } fn walk_macro (& mut self , i : & 'ast syn :: Macro) { syn :: visit :: visit_macro (self , i) ; } fn walk_type_path (& mut self , i : & 'ast syn :: TypePath) { syn :: visit :: visit_type_path (self , i) ; } fn walk_type_param_bound (& mut self , i : & 'ast syn :: TypeParamBound) { syn :: visit :: visit_type_param_bound (self , i) ; } fn walk_variant (& mut self , _i : & 'ast syn :: Variant) { } fn walk_trait_item (& mut self , _i : & 'ast syn :: TraitItem) { } fn walk_expr (& mut self , _i : & 'ast syn :: Expr) { } }
    };
}

impl_172!()