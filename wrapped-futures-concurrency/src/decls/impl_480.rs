macro_rules! deps {
    () => {
        IntoStream!();
        Zip!();
    };
}

macro_rules! impl_480 {
    () => {
        deps!();
        impl < S , const N : usize > ZipTrait for [S ; N] where S : IntoStream , { type Item = < Zip < S :: IntoStream , N > as Stream > :: Item ; type Stream = Zip < S :: IntoStream , N > ; fn zip (self) -> Self :: Stream { Zip :: new (self . map (| i | i . into_stream ())) } }
    };
}

impl_480!()