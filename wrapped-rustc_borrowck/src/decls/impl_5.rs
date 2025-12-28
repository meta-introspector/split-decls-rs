macro_rules! deps {
    () => {
        TwoPhaseActivation!();
        BorrowData!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < 'tcx > BorrowData < 'tcx > { pub fn reserve_location (& self) -> Location { self . reserve_location } pub fn activation_location (& self) -> TwoPhaseActivation { self . activation_location } pub fn kind (& self) -> mir :: BorrowKind { self . kind } pub fn region (& self) -> RegionVid { self . region } pub fn borrowed_place (& self) -> mir :: Place < 'tcx > { self . borrowed_place } pub fn assigned_place (& self) -> mir :: Place < 'tcx > { self . assigned_place } }
    };
}

impl_5!();