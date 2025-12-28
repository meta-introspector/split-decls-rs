macro_rules! deps {
    () => {
        TraitItem!();
        FnSig!();
        TraitFn!();
        Ty!();
        TraitItemId!();
        BodyId!();
        GenericBounds!();
        TraitItemKind!();
    };
}

macro_rules! impl_246 {
    () => {
        deps!();
        impl < 'hir > TraitItem < 'hir > { # [inline] pub fn hir_id (& self) -> HirId { HirId :: make_owner (self . owner_id . def_id) } pub fn trait_item_id (& self) -> TraitItemId { TraitItemId { owner_id : self . owner_id } } expect_methods_self_kind ! { expect_const , (&'hir Ty <'hir >, Option < BodyId >) , TraitItemKind :: Const (ty , body) , (ty , * body) ; expect_fn , (& FnSig <'hir >, & TraitFn <'hir >) , TraitItemKind :: Fn (ty , trfn) , (ty , trfn) ; expect_type , (GenericBounds <'hir >, Option <&'hir Ty <'hir >>) , TraitItemKind :: Type (bounds , ty) , (bounds , * ty) ; } }
    };
}

impl_246!()