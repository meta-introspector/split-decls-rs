macro_rules! deps {
    () => {
        OwnerNode!();
        OwnerNodes!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl < 'tcx > OwnerNodes < 'tcx > { pub fn node (& self) -> OwnerNode < 'tcx > { self . nodes [ItemLocalId :: ZERO] . node . as_owner () . unwrap () } }
    };
}

impl_169!()