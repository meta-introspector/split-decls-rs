macro_rules! deps {
    () => {
        ForeignItem!();
        ForeignItemId!();
    };
}

macro_rules! impl_329 {
    () => {
        deps!();
        impl ForeignItem < '_ > { # [inline] pub fn hir_id (& self) -> HirId { HirId :: make_owner (self . owner_id . def_id) } pub fn foreign_item_id (& self) -> ForeignItemId { ForeignItemId { owner_id : self . owner_id } } }
    };
}

impl_329!()