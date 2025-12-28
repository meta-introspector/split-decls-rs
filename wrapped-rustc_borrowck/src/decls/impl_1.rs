macro_rules! deps {
    () => {
        LocalsStateAtExit!();
        BorrowData!();
        BorrowSet!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl < 'tcx > BorrowSet < 'tcx > { pub fn location_map (& self) -> & FxIndexMap < Location , BorrowData < 'tcx > > { & self . location_map } pub fn activation_map (& self) -> & FxIndexMap < Location , Vec < BorrowIndex > > { & self . activation_map } pub fn local_map (& self) -> & FxIndexMap < mir :: Local , FxIndexSet < BorrowIndex > > { & self . local_map } pub fn locals_state_at_exit (& self) -> & LocalsStateAtExit { & self . locals_state_at_exit } }
    };
}

impl_1!()