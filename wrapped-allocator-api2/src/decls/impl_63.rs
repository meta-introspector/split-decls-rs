macro_rules! deps {
    () => {
        Allocator!();
        Box!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < T : ? Sized , A : Allocator > borrow :: BorrowMut < T > for Box < T , A > { # [inline (always)] fn borrow_mut (& mut self) -> & mut T { self } }
    };
}

impl_63!()