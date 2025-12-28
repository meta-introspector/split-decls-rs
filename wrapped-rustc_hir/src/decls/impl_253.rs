macro_rules! deps {
    () => {
        ImplItemId!();
        ImplItemKind!();
        ImplItemImplKind!();
        Ty!();
        ImplItem!();
        BodyId!();
        FnSig!();
    };
}

macro_rules! impl_253 {
    () => {
        deps!();
        impl < 'hir > ImplItem < 'hir > { # [inline] pub fn hir_id (& self) -> HirId { HirId :: make_owner (self . owner_id . def_id) } pub fn impl_item_id (& self) -> ImplItemId { ImplItemId { owner_id : self . owner_id } } pub fn vis_span (& self) -> Option < Span > { match self . impl_kind { ImplItemImplKind :: Trait { .. } => None , ImplItemImplKind :: Inherent { vis_span , .. } => Some (vis_span) , } } expect_methods_self_kind ! { expect_const , (&'hir Ty <'hir >, BodyId) , ImplItemKind :: Const (ty , body) , (ty , * body) ; expect_fn , (& FnSig <'hir >, BodyId) , ImplItemKind :: Fn (ty , body) , (ty , * body) ; expect_type , &'hir Ty <'hir >, ImplItemKind :: Type (ty) , ty ; } }
    };
}

impl_253!()