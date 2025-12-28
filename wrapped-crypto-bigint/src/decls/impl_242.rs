macro_rules! deps {
    () => {
        Odd!();
    };
}

macro_rules! impl_242 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl PartialEq < Odd < BoxedUint > > for BoxedUint { fn eq (& self , other : & Odd < BoxedUint >) -> bool { self . eq (& other . 0) } }
    };
}

impl_242!()