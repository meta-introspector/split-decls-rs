macro_rules! deps {
    () => {
        Pointable!();
        Owned!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < T : ? Sized + Pointable > BorrowMut < T > for Owned < T > { fn borrow_mut (& mut self) -> & mut T { self . deref_mut () } }
    };
}

impl_46!()