macro_rules! deps {
    () => {
        AsyncAsSync!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl < T > AsMut < T > for AsyncAsSync < '_ , '_ , T > { # [inline] fn as_mut (& mut self) -> & mut T { & mut self . inner } }
    };
}

impl_205!();