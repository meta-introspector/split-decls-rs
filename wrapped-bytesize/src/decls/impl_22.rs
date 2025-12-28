macro_rules! deps {
    () => {
        ByteSize!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl Serialize for ByteSize { fn serialize < S > (& self , ser : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { if ser . is_human_readable () { < String > :: serialize (& self . to_string () , ser) } else { self . 0 . serialize (ser) } } }
    };
}

impl_22!()