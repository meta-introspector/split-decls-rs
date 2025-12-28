macro_rules! deps {
    () => {
        BitSetByteSerializer!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < 'a > Serialize for BitSetByteSerializer < 'a > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let len = self . 0 . as_slice () . len () * BYTES ; let mut temp = Vec :: with_capacity (len) ; for block in self . 0 . as_slice () { temp . extend (& block . to_le_bytes ()) ; } serializer . serialize_bytes (& temp) } }
    };
}

impl_71!();