macro_rules! deps {
    () => {
        HirId!();
        OwnerId!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl From < OwnerId > for HirId { fn from (owner : OwnerId) -> HirId { HirId { owner , local_id : ItemLocalId :: ZERO } } }
    };
}

impl_2!()