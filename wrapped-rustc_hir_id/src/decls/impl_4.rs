macro_rules! deps {
    () => {
        OwnerId!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl OwnerId { # [inline] pub fn to_def_id (self) -> DefId { self . def_id . to_def_id () } }
    };
}

impl_4!();