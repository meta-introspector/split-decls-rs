macro_rules! deps {
    () => {
        WithContext!();
        BorrowDecoder!();
    };
}

macro_rules! impl_280 {
    () => {
        deps!();
        impl < 'de , C , D : BorrowDecoder < 'de > > BorrowDecoder < 'de > for WithContext < '_ , D , C > { type BR = D :: BR ; fn borrow_reader (& mut self) -> & mut Self :: BR { self . decoder . borrow_reader () } }
    };
}

impl_280!();