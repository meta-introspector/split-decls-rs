macro_rules! deps {
    () => {
        OwnerId!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl From < OwnerId > for DefId { fn from (value : OwnerId) -> Self { value . to_def_id () } }
    };
}

impl_3!();