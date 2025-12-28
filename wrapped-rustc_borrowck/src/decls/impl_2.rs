macro_rules! deps {
    () => {
        BorrowSet!();
        BorrowData!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl < 'tcx > Index < BorrowIndex > for BorrowSet < 'tcx > { type Output = BorrowData < 'tcx > ; fn index (& self , index : BorrowIndex) -> & BorrowData < 'tcx > { & self . location_map [index . as_usize ()] } }
    };
}

impl_2!()