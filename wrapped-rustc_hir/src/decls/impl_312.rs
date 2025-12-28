macro_rules! deps {
    () => {
        UseKind!();
        VariantData!();
        InlineAsm!();
        Generics!();
        ItemId!();
        Ty!();
        EnumDef!();
        Constness!();
        Safety!();
        ForeignItemId!();
        GenericBounds!();
        UsePath!();
        MacroKinds!();
        Item!();
        FnSig!();
        ItemKind!();
        TraitItemId!();
        BodyId!();
        Impl!();
        Mod!();
    };
}

macro_rules! impl_312 {
    () => {
        deps!();
        impl < 'hir > Item < 'hir > { # [inline] pub fn hir_id (& self) -> HirId { HirId :: make_owner (self . owner_id . def_id) } pub fn item_id (& self) -> ItemId { ItemId { owner_id : self . owner_id } } # [doc = " Check if this is an [`ItemKind::Enum`], [`ItemKind::Struct`] or"] # [doc = " [`ItemKind::Union`]."] pub fn is_adt (& self) -> bool { matches ! (self . kind , ItemKind :: Enum (..) | ItemKind :: Struct (..) | ItemKind :: Union (..)) } # [doc = " Check if this is an [`ItemKind::Struct`] or [`ItemKind::Union`]."] pub fn is_struct_or_union (& self) -> bool { matches ! (self . kind , ItemKind :: Struct (..) | ItemKind :: Union (..)) } expect_methods_self_kind ! { expect_extern_crate , (Option < Symbol >, Ident) , ItemKind :: ExternCrate (s , ident) , (* s , * ident) ; expect_use , (&'hir UsePath <'hir >, UseKind) , ItemKind :: Use (p , uk) , (p , * uk) ; expect_static , (Mutability , Ident , &'hir Ty <'hir >, BodyId) , ItemKind :: Static (mutbl , ident , ty , body) , (* mutbl , * ident , ty , * body) ; expect_const , (Ident , &'hir Generics <'hir >, &'hir Ty <'hir >, BodyId) , ItemKind :: Const (ident , generics , ty , body) , (* ident , generics , ty , * body) ; expect_fn , (Ident , & FnSig <'hir >, &'hir Generics <'hir >, BodyId) , ItemKind :: Fn { ident , sig , generics , body , .. } , (* ident , sig , generics , * body) ; expect_macro , (Ident , & ast :: MacroDef , MacroKinds) , ItemKind :: Macro (ident , def , mk) , (* ident , def , * mk) ; expect_mod , (Ident , &'hir Mod <'hir >) , ItemKind :: Mod (ident , m) , (* ident , m) ; expect_foreign_mod , (ExternAbi , &'hir [ForeignItemId]) , ItemKind :: ForeignMod { abi , items } , (* abi , items) ; expect_global_asm , &'hir InlineAsm <'hir >, ItemKind :: GlobalAsm { asm , .. } , asm ; expect_ty_alias , (Ident , &'hir Generics <'hir >, &'hir Ty <'hir >) , ItemKind :: TyAlias (ident , generics , ty) , (* ident , generics , ty) ; expect_enum , (Ident , &'hir Generics <'hir >, & EnumDef <'hir >) , ItemKind :: Enum (ident , generics , def) , (* ident , generics , def) ; expect_struct , (Ident , &'hir Generics <'hir >, & VariantData <'hir >) , ItemKind :: Struct (ident , generics , data) , (* ident , generics , data) ; expect_union , (Ident , &'hir Generics <'hir >, & VariantData <'hir >) , ItemKind :: Union (ident , generics , data) , (* ident , generics , data) ; expect_trait , (Constness , IsAuto , Safety , Ident , &'hir Generics <'hir >, GenericBounds <'hir >, &'hir [TraitItemId]) , ItemKind :: Trait (constness , is_auto , safety , ident , generics , bounds , items) , (* constness , * is_auto , * safety , * ident , generics , bounds , items) ; expect_trait_alias , (Ident , &'hir Generics <'hir >, GenericBounds <'hir >) , ItemKind :: TraitAlias (ident , generics , bounds) , (* ident , generics , bounds) ; expect_impl , & Impl <'hir >, ItemKind :: Impl (imp) , imp ; } }
    };
}

impl_312!()