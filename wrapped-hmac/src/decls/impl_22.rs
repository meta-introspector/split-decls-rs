macro_rules! deps {
    () => {
        HmacResetCore!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < D : EagerHash > Reset for HmacResetCore < D > { # [inline (always)] fn reset (& mut self) { self . digest = self . ipad_digest . clone () ; } }
    };
}

impl_22!();