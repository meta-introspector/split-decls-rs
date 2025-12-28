macro_rules! deps {
    () => {
        ExtendElement!();
        ExtendWith!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl < T : Clone > ExtendWith < T > for ExtendElement < T > { # [inline (always)] fn next (& mut self) -> T { self . 0 . clone () } # [inline (always)] fn last (self) -> T { self . 0 } }
    };
}

impl_156!()