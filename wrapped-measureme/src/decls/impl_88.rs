macro_rules! deps {
    () => {
        SerializableString!();
        StringComponent!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl < 'a > SerializableString for [StringComponent < 'a >] { # [inline] fn serialized_size (& self) -> usize { self . iter () . map (| c | c . serialized_size ()) . sum :: < usize > () + 1 } # [inline] fn serialize (& self , mut bytes : & mut [u8]) { assert ! (bytes . len () == self . serialized_size ()) ; for component in self . iter () { bytes = component . serialize (bytes) ; } assert ! (bytes . len () == 1) ; bytes [0] = TERMINATOR ; } }
    };
}

impl_88!();