macro_rules! deps {
    () => {
        Odd!();
    };
}

macro_rules! impl_243 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl PartialOrd < Odd < BoxedUint > > for BoxedUint { fn partial_cmp (& self , other : & Odd < BoxedUint >) -> Option < Ordering > { Some (self . cmp (& other . 0)) } }
    };
}

impl_243!()