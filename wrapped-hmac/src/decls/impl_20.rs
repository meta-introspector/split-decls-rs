macro_rules! deps {
    () => {
        HmacResetCore!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < D : EagerHash > UpdateCore for HmacResetCore < D > { # [inline (always)] fn update_blocks (& mut self , blocks : & [Block < Self >]) { self . digest . update_blocks (blocks) ; } }
    };
}

impl_20!();