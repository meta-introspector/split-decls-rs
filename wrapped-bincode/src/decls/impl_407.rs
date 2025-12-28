macro_rules! deps {
    () => {
        BorrowDecoder!();
    };
}

macro_rules! impl_407 {
    () => {
        deps!();
        impl < 'de , T > BorrowDecoder < 'de > for & mut T where T : BorrowDecoder < 'de > , { type BR = T :: BR ; fn borrow_reader (& mut self) -> & mut Self :: BR { T :: borrow_reader (self) } }
    };
}

impl_407!()