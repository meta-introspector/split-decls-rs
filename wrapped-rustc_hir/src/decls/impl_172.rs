macro_rules! deps {
    () => {
        OwnerInfo!();
        OwnerNode!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        impl < 'tcx > OwnerInfo < 'tcx > { # [inline] pub fn node (& self) -> OwnerNode < 'tcx > { self . nodes . node () } }
    };
}

impl_172!()