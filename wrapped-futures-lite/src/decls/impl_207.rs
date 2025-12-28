macro_rules! deps {
    () => {
        AsyncAsSync!();
    };
}

macro_rules! impl_207 {
    () => {
        deps!();
        impl < T > BorrowMut < T > for AsyncAsSync < '_ , '_ , T > { # [inline] fn borrow_mut (& mut self) -> & mut T { & mut self . inner } }
    };
}

impl_207!()