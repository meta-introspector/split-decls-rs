macro_rules! deps {
    () => {
        TraitItemId!();
    };
}

macro_rules! impl_241 {
    () => {
        deps!();
        impl TraitItemId { # [inline] pub fn hir_id (& self) -> HirId { HirId :: make_owner (self . owner_id . def_id) } }
    };
}

impl_241!();