macro_rules! deps {
    () => {
        ForeignItemId!();
        BodyId!();
        Body!();
        Item!();
        ItemId!();
        HirTyCtxt!();
        TraitItemId!();
        ImplItemId!();
        Node!();
        TraitItem!();
        ImplItem!();
        ForeignItem!();
    };
}

macro_rules! impl_350 {
    () => {
        deps!();
        impl < 'hir > HirTyCtxt < 'hir > for ! { fn hir_node (& self , _ : HirId) -> Node < 'hir > { unreachable ! () ; } fn hir_body (& self , _ : BodyId) -> & 'hir Body < 'hir > { unreachable ! () ; } fn hir_item (& self , _ : ItemId) -> & 'hir Item < 'hir > { unreachable ! () ; } fn hir_trait_item (& self , _ : TraitItemId) -> & 'hir TraitItem < 'hir > { unreachable ! () ; } fn hir_impl_item (& self , _ : ImplItemId) -> & 'hir ImplItem < 'hir > { unreachable ! () ; } fn hir_foreign_item (& self , _ : ForeignItemId) -> & 'hir ForeignItem < 'hir > { unreachable ! () ; } }
    };
}

impl_350!();