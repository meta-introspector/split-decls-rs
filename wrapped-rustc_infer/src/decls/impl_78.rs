macro_rules! deps {
    () => {
        OpaqueTypeTable!();
        OpaqueTypeStorage!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < 'tcx > Deref for OpaqueTypeTable < '_ , 'tcx > { type Target = OpaqueTypeStorage < 'tcx > ; fn deref (& self) -> & Self :: Target { self . storage } }
    };
}

impl_78!();