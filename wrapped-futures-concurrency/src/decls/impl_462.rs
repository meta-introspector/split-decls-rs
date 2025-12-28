macro_rules! deps {
    () => {
        Merge!();
        IntoStream!();
    };
}

macro_rules! impl_462 {
    () => {
        deps!();
        impl < S > MergeTrait for Vec < S > where S : IntoStream , { type Item = < Merge < S :: IntoStream > as Stream > :: Item ; type Stream = Merge < S :: IntoStream > ; fn merge (self) -> Self :: Stream { Merge :: new (self . into_iter () . map (| i | i . into_stream ()) . collect ()) } }
    };
}

impl_462!()