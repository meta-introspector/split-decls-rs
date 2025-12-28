macro_rules! deps {
    () => {
        ErrorPositions!();
        Error!();
        Result!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl Serialize for ErrorPositions { fn serialize < S : Serializer > (& self , serializer : S) -> std :: result :: Result < S :: Ok , S :: Error > { serializer . collect_seq (self . clone ()) } }
    };
}

impl_15!()