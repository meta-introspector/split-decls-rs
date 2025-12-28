macro_rules! deps {
    () => {
        ExtendInto!();
    };
}

macro_rules! impl_355 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl ExtendInto for char { type Item = char ; type Extender = String ; # [inline] fn new_builder (& self) -> String { String :: new () } # [inline] fn extend_into (& self , acc : & mut String) { acc . push (* self) ; } }
    };
}

impl_355!()