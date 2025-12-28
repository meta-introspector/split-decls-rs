macro_rules! deps {
    () => {
        ForeignItemId!();
    };
}

macro_rules! impl_327 {
    () => {
        deps!();
        impl ForeignItemId { # [inline] pub fn hir_id (& self) -> HirId { HirId :: make_owner (self . owner_id . def_id) } }
    };
}

impl_327!()