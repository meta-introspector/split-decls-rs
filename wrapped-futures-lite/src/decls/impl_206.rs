macro_rules! deps {
    () => {
        AsyncAsSync!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl < T > Borrow < T > for AsyncAsSync < '_ , '_ , T > { # [inline] fn borrow (& self) -> & T { & self . inner } }
    };
}

impl_206!()