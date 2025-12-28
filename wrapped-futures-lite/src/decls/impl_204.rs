macro_rules! deps {
    () => {
        AsyncAsSync!();
    };
}

macro_rules! impl_204 {
    () => {
        deps!();
        impl < T > AsRef < T > for AsyncAsSync < '_ , '_ , T > { # [inline] fn as_ref (& self) -> & T { & self . inner } }
    };
}

impl_204!()