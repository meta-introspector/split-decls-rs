macro_rules! deps {
    () => {
        Error!();
        Result!();
        ErrorPositions!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl Serialize for ErrorPositions { fn serialize < S : Serializer > (& self , serializer : S) -> std :: result :: Result < S :: Ok , S :: Error > { serializer . collect_seq (self . clone ()) } }
    };
}

impl_146!()