macro_rules! deps {
    () => {
        Zip!();
        IntoStream!();
    };
}

macro_rules! impl_504 {
    () => {
        deps!();
        impl < S > ZipTrait for Vec < S > where S : IntoStream , { type Item = < Zip < S :: IntoStream > as Stream > :: Item ; type Stream = Zip < S :: IntoStream > ; fn zip (self) -> Self :: Stream { Zip :: new (self . into_iter () . map (| i | i . into_stream ()) . collect ()) } }
    };
}

impl_504!();