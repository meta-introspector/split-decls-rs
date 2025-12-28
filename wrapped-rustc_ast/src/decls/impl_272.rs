macro_rules! deps {
    () => {
        Attribute!();
        MarkedAttrs!();
    };
}

macro_rules! impl_272 {
    () => {
        deps!();
        impl MarkedAttrs { pub fn new () -> Self { MarkedAttrs (GrowableBitSet :: new_empty ()) } pub fn mark (& mut self , attr : & Attribute) { self . 0 . insert (attr . id) ; } pub fn is_marked (& self , attr : & Attribute) -> bool { self . 0 . contains (attr . id) } }
    };
}

impl_272!()