macro_rules! deps {
    () => {
        ImplItemId!();
    };
}

macro_rules! impl_250 {
    () => {
        deps!();
        impl ImplItemId { # [inline] pub fn hir_id (& self) -> HirId { HirId :: make_owner (self . owner_id . def_id) } }
    };
}

impl_250!()