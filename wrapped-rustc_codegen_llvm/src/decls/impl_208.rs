macro_rules! deps {
    () => {
        FullCx!();
        SCx!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl < 'll > Borrow < SCx < 'll > > for FullCx < 'll , '_ > { fn borrow (& self) -> & SCx < 'll > { & self . scx } }
    };
}

impl_208!();