macro_rules! deps {
    () => {
        ExtendInto!();
    };
}

macro_rules! impl_351 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl ExtendInto for [u8] { type Item = u8 ; type Extender = Vec < u8 > ; # [inline] fn new_builder (& self) -> Vec < u8 > { Vec :: new () } # [inline] fn extend_into (& self , acc : & mut Vec < u8 >) { acc . extend (self . iter () . cloned ()) ; } }
    };
}

impl_351!()