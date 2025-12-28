macro_rules! deps {
    () => {
        ExtendInto!();
    };
}

macro_rules! impl_354 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl ExtendInto for & str { type Item = char ; type Extender = String ; # [inline] fn new_builder (& self) -> String { String :: new () } # [inline] fn extend_into (& self , acc : & mut String) { acc . push_str (self) ; } }
    };
}

impl_354!();