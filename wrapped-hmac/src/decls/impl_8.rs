macro_rules! deps {
    () => {
        HmacCore!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < D : EagerHash > UpdateCore for HmacCore < D > { # [inline (always)] fn update_blocks (& mut self , blocks : & [Block < Self >]) { self . digest . update_blocks (blocks) ; } }
    };
}

impl_8!()