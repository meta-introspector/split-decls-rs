macro_rules! deps {
    () => {
        ItemId!();
    };
}

macro_rules! impl_310 {
    () => {
        deps!();
        impl ItemId { # [inline] pub fn hir_id (& self) -> HirId { HirId :: make_owner (self . owner_id . def_id) } }
    };
}

impl_310!();